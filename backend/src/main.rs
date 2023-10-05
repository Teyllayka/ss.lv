use actix_cors::Cors;
use dotenvy::dotenv;
use std::{
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};

use actix_web::{guard, http, web, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Object, Schema, SimpleObject};
use async_graphql_actix_web::GraphQL;
use chrono::Utc;
use entity::advert::{self, Entity as Advert};
use entity::user::{self, Entity as User};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait, ModelTrait, Set};
use sha2::Sha256;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

const ACCESS_EXPIRATION: usize = 10;
const REFRESH_EXPIRATION: usize = 180;

#[derive(SimpleObject)]
#[graphql(name = "LoginResponse")]
pub struct LoginResponse {
    refresh_token: String,
    access_token: String,
}

pub struct Context {
    pub db: DatabaseConnection,
    pub access_key: Hmac<Sha256>,
    pub refresh_key: Hmac<Sha256>,
}

impl Context {
    pub fn new(
        db: DatabaseConnection,
        access_key: Hmac<Sha256>,
        refresh_key: Hmac<Sha256>,
    ) -> Self {
        Self {
            db,
            access_key,
            refresh_key,
        }
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_user(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
    ) -> Result<user::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let user: Option<user::Model> = User::find_by_id(id).one(&my_ctx.db).await?;

        let mut user = match user {
            Some(user) => user,
            None => {
                return Err(async_graphql::Error::new(
                    "Wrong email or password".to_string(),
                ))
            }
        };

        let adverts: Vec<advert::Model> = user.find_related(Advert).all(&my_ctx.db).await?;

        user.adverts = adverts;

        return Ok(user);
    }

    async fn get_advert(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
    ) -> Result<advert::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let advert: Option<advert::Model> = Advert::find_by_id(id).one(&my_ctx.db).await?;

        let advert = match advert {
            Some(advert) => advert,
            None => return Err(async_graphql::Error::new("advert not found".to_string())),
        };

        return Ok(advert);
    }

    async fn get_adverts(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Result<Vec<advert::Model>, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let advert: Option<Vec<advert::Model>> = Some(Advert::find().all(&my_ctx.db).await?);

        let advert = match advert {
            Some(advert) => advert,
            None => return Err(async_graphql::Error::new("advert not found".to_string())),
        };

        // You can now access the database connection via `&my_ctx.db`
        return Ok(advert);
    }

    async fn me(
        &self,
        ctx: &async_graphql::Context<'_>,
        access_token: String,
    ) -> Result<user::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let claims: BTreeMap<String, String> =
            match access_token.verify_with_key(&my_ctx.access_key) {
                Ok(res) => res,
                Err(err) => return Err(async_graphql::Error::new(err.to_string())),
            };
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        if claims["sub"] == "someone" && claims["exp"].parse::<usize>().unwrap() >= now {
            println!("{}, {}", claims["exp"], now);
            let id: i32 = claims["id"].parse().unwrap();
            let user: Option<user::Model> = User::find_by_id(id).one(&my_ctx.db).await?;

            let mut user = match user {
                Some(user) => user,
                None => return Err(async_graphql::Error::new("Wrong token".to_string())),
            };

            let adverts: Vec<advert::Model> = user.find_related(Advert).all(&my_ctx.db).await?;

            user.adverts = adverts;

            return Ok(user);
        } else {
            return Err(async_graphql::Error::new(
                "you are not loged in".to_string(),
            ));
        }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn register(
        &self,
        ctx: &async_graphql::Context<'_>,
        email: String,
        password: String,
        surname: String,
        name: String,
        phone: String,
    ) -> Result<user::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(hash) => hash.to_string(),
            Err(err) => {
                return Err(async_graphql::Error::new(err.to_string()));
            }
        };

        let parsed_hash = PasswordHash::new(&password_hash)
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        let naive_date_time = Utc::now().naive_utc();

        let user = user::ActiveModel {
            name: Set(name),
            surname: Set(surname),
            email: Set(email),
            phone: Set(phone),
            password_hash: Set(parsed_hash.to_string()),
            created_at: Set(naive_date_time),
            updated_at: Set(naive_date_time),
            refresh_token: Set(None),
            balance: Set(0.0),
            ..Default::default()
        };

        let user: user::Model = user.insert(&my_ctx.db).await?;

        return Ok(user);
    }

    async fn login(
        &self,
        ctx: &async_graphql::Context<'_>,
        email: String,
        password: String,
    ) -> Result<LoginResponse, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let user: Option<user::Model> = User::find_by_email(email).one(&my_ctx.db).await?;

        let user = match user {
            Some(user) => user,
            None => {
                return Err(async_graphql::Error::new(
                    "Wrong email or password".to_string(),
                ))
            }
        };

        let argon2 = Argon2::default();

        let response = argon2
            .verify_password(
                password.as_bytes(),
                &PasswordHash::new(&user.password_hash).unwrap(),
            )
            .is_ok();

        if response {
            let mut refresh_claims = BTreeMap::new();
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize;
            let expiration = now + (ACCESS_EXPIRATION * 60); // 1 minutes from now
            let expiration = expiration.to_string();
            let expiration2 = now + (REFRESH_EXPIRATION * 60); // 60 minutes from now
            let expiration2 = expiration2.to_string();

            let id = user.id.to_string();
            let email = user.email.to_string();

            refresh_claims.insert("sub", "someone");
            refresh_claims.insert("id", &id);
            refresh_claims.insert("email", &email);
            refresh_claims.insert("exp", &expiration2);

            let refresh_token = match refresh_claims.clone().sign_with_key(&my_ctx.refresh_key) {
                Ok(token) => token,
                Err(err) => return Err(async_graphql::Error::new(err.to_string())),
            };

            let mut access_claims = BTreeMap::new();
            access_claims.insert("sub", "someone");
            access_claims.insert("id", &id);
            access_claims.insert("email", &email);
            access_claims.insert("exp", &expiration);
            let access_token = match access_claims.sign_with_key(&my_ctx.access_key) {
                Ok(token) => token,
                Err(err) => return Err(async_graphql::Error::new(err.to_string())),
            };

            user::ActiveModel {
                id: Set(user.id),
                refresh_token: Set(Some(refresh_token.clone())),
                ..Default::default()
            }
            .update(&my_ctx.db)
            .await?;

            Ok(LoginResponse {
                refresh_token,
                access_token,
            })
        } else {
            return Err(async_graphql::Error::new(
                "Wrong email or password".to_string(),
            ));
        }
    }

    async fn refresh(
        &self,
        ctx: &async_graphql::Context<'_>,
        refresh_token: String,
    ) -> Result<LoginResponse, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let claims: BTreeMap<String, String> =
            match refresh_token.verify_with_key(&my_ctx.refresh_key) {
                Ok(res) => res,
                Err(err) => {
                    return Err(async_graphql::Error::new(err.to_string()));
                }
            };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        if claims["sub"] == "someone" && claims["exp"].parse::<usize>().unwrap() >= now {
            let id = claims["id"].parse::<i32>().unwrap();

            let user: Option<user::Model> = User::find_by_id(id).one(&my_ctx.db).await?;

            let user = match user {
                Some(user) => user,
                None => return Err(async_graphql::Error::new("Wrong token".to_string())),
            };

            if user.refresh_token == Some(refresh_token.clone()) {
                let mut refresh_claims = BTreeMap::new();
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as usize;
                let expiration = now + (ACCESS_EXPIRATION * 60); // 1 minutes from now
                let expiration = expiration.to_string();
                let expiration2 = now + (REFRESH_EXPIRATION * 60); // 60 minutes from now
                let expiration2 = expiration2.to_string();

                let id = user.id.to_string();
                let email = user.email.to_string();

                refresh_claims.insert("sub", "someone");
                refresh_claims.insert("id", &id);
                refresh_claims.insert("email", &email);
                refresh_claims.insert("exp", &expiration2);

                let refresh_token = match refresh_claims.clone().sign_with_key(&my_ctx.refresh_key)
                {
                    Ok(token) => token,
                    Err(err) => return Err(async_graphql::Error::new(err.to_string())),
                };

                let mut access_claims = BTreeMap::new();
                access_claims.insert("sub", "someone");
                access_claims.insert("id", &id);
                access_claims.insert("email", &email);
                access_claims.insert("exp", &expiration);
                let access_token = match access_claims.sign_with_key(&my_ctx.access_key) {
                    Ok(token) => token,
                    Err(err) => return Err(async_graphql::Error::new(err.to_string())),
                };

                user::ActiveModel {
                    id: Set(user.id),
                    refresh_token: Set(Some(refresh_token.clone())),
                    ..Default::default()
                }
                .update(&my_ctx.db)
                .await?;

                return Ok(LoginResponse {
                    refresh_token,
                    access_token,
                });
            } else {
                return Err(async_graphql::Error::new("Wrong token".to_string()));
            }
        } else {
            return Err(async_graphql::Error::new("Wrong token".to_string()));
        }
    }

    async fn create_advert(
        &self,
        ctx: &async_graphql::Context<'_>,
        access_token: String,
        price: f32,
        location: String,
    ) -> Result<advert::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let claims: BTreeMap<String, String> =
            match access_token.verify_with_key(&my_ctx.access_key) {
                Ok(res) => res,
                Err(err) => return Err(async_graphql::Error::new(err.to_string())),
            };
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        if claims["sub"] == "someone" && claims["exp"].parse::<usize>().unwrap() >= now {
            let naive_date_time = Utc::now().naive_utc();
            let advert = advert::ActiveModel {
                available: Set(true),
                user_id: Set(claims["id"].parse().unwrap()),
                created_at: Set(naive_date_time),
                updated_at: Set(naive_date_time),
                price: Set(price),
                location: Set(location),
                ..Default::default()
            };
            let advert: advert::Model = advert.insert(&my_ctx.db).await?;
            return Ok(advert);
        } else {
            return Err(async_graphql::Error::new("Wrong token".to_string()));
        }
    }
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env file not found");
    let db_url = dotenvy::var("DATABASE_URL").expect("HOME environment variable not found");
    let refresh_secret =
        dotenvy::var("REFRESH_SECRET").expect("HOME environment variable not found");
    let access_secret = dotenvy::var("ACCESS_SECRET").expect("HOME environment variable not found");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    let db: DatabaseConnection = Database::connect(db_url)
        .await
        .expect("error with connection");

    Migrator::up(&db, None).await.expect("migration ban");

    println!("GraphiQL IDE: http://localhost:8000");

    let access_key: Hmac<Sha256> = Hmac::new_from_slice(access_secret.as_bytes()).unwrap();
    let refresh_key: Hmac<Sha256> = Hmac::new_from_slice(refresh_secret.as_bytes()).unwrap();

    HttpServer::new(move || {
        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
            .data(Context::new(
                db.clone(),
                access_key.clone(),
                refresh_key.clone(),
            ))
            .finish();
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .to(GraphQL::new(schema)),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
