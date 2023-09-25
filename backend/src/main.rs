use actix_cors::Cors;
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
use sea_orm::{
    ActiveModelTrait, Database, DatabaseConnection, DbErr, EntityTrait, ModelTrait, Set,
};
use sha2::Sha256;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(SimpleObject)]
#[graphql(name = "UserWithAdverts")]
pub struct UserWithAdverts {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub phone: String,
    adverts: Vec<advert::Model>,
}

#[derive(SimpleObject)]
#[graphql(name = "LoginResponse")]
pub struct LoginResponse {
    refresh_token: String,
    access_token: String,
}

pub struct Context {
    pub db: DatabaseConnection,
}

impl Context {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_user(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
    ) -> Result<UserWithAdverts, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let user: Option<user::Model> = User::find_by_id(id).one(&my_ctx.db).await?;

        let user = match user {
            Some(user) => user,
            None => {
                return Err(async_graphql::Error::new(
                    "Wrong email or password".to_string(),
                ))
            }
        };

        let adverts: Vec<advert::Model> = user.find_related(Advert).all(&my_ctx.db).await?;
        println!("{:?}", adverts);

        // You can now access the database connection via `my_ctx.db`
        return Ok(UserWithAdverts {
            id: user.id,
            name: user.name,
            surname: user.surname,
            email: user.email,
            phone: user.phone,
            adverts: adverts,
        });
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

        // You can now access the database connection via `my_ctx.db`
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

        // You can now access the database connection via `my_ctx.db`
        return Ok(advert);
    }

    async fn me(
        &self,
        ctx: &async_graphql::Context<'_>,
        access_token: String,
    ) -> Result<user::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();
        let key: Hmac<Sha256> = match Hmac::new_from_slice(b"some-secret2") {
            Ok(key) => key,
            Err(err) => return Err(async_graphql::Error::new("Wrong token".to_string())),
        };
        let claims: BTreeMap<String, String> = match access_token.verify_with_key(&key) {
            Ok(res) => res,
            Err(err) => return Err(async_graphql::Error::new("Wrong token".to_string())),
        };
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        if claims["sub"] == "someone" && claims["exp"].parse::<usize>().unwrap() >= now {
            let id: i32 = claims["id"].parse().unwrap();
            let user: Option<user::Model> = User::find_by_id(id).one(&my_ctx.db).await?;

            let user = match user {
                Some(user) => user,
                None => return Err(async_graphql::Error::new("Wrong token".to_string())),
            };

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
            let refresh_key: Hmac<Sha256> = match Hmac::new_from_slice(b"some-secret") {
                Ok(key) => key,
                Err(err) => return Err(async_graphql::Error::new("internal error".to_string())),
            };

            let access_key: Hmac<Sha256> = match Hmac::new_from_slice(b"some-secret2") {
                Ok(key) => key,
                Err(err) => return Err(async_graphql::Error::new("internal error".to_string())),
            };

            let mut refresh_claims = BTreeMap::new();
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize;
            let expiration = now + (1 * 60); // 1 minutes from now
            let expiration = expiration.to_string();
            let expiration2 = now + (60 * 60); // 60 minutes from now
            let expiration2 = expiration2.to_string();

            let id = user.id.to_string();
            let email = user.email.to_string();

            refresh_claims.insert("sub", "someone");
            refresh_claims.insert("id", &id);
            refresh_claims.insert("email", &email);
            refresh_claims.insert("exp", &expiration2);

            let refresh_token = match refresh_claims.clone().sign_with_key(&refresh_key) {
                Ok(token) => token,
                Err(err) => return Err(async_graphql::Error::new("internal error".to_string())),
            };

            let mut access_claims = BTreeMap::new();
            access_claims.insert("sub", "someone");
            access_claims.insert("id", &id);
            access_claims.insert("email", &email);
            access_claims.insert("exp", &expiration);
            let access_token = match access_claims.sign_with_key(&access_key) {
                Ok(token) => token,
                Err(err) => return Err(async_graphql::Error::new("internal error".to_string())),
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

        let refresh_key: Hmac<Sha256> = match Hmac::new_from_slice(b"some-secret") {
            Ok(key) => key,
            Err(err) => {
                return Err(async_graphql::Error::new("Wrong token".to_string()));
            }
        };

        let claims: BTreeMap<String, String> =
            match refresh_token.clone().verify_with_key(&refresh_key) {
                Ok(res) => res,
                Err(err) => {
                    return Err(async_graphql::Error::new("Wrong token".to_string()));
                }
            };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        if claims["sub"] == "someone" && claims["exp"].parse::<usize>().unwrap() >= now {
            let user: Option<user::Model> = User::find_by_id(1).one(&my_ctx.db).await?;

            let user = match user {
                Some(user) => user,
                None => return Err(async_graphql::Error::new("Wrong token".to_string())),
            };

            if user.refresh_token == Some(refresh_token.clone()) {
                let access_key: Hmac<Sha256> = match Hmac::new_from_slice(b"some-secret2") {
                    Ok(key) => key,
                    Err(err) => return Err(async_graphql::Error::new("Wrong token".to_string())),
                };

                let mut refresh_claims = BTreeMap::new();
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as usize;
                let expiration = now + (1 * 60); // 1 minutes from now
                let expiration = expiration.to_string();
                let expiration2 = now + (60 * 60); // 60 minutes from now
                let expiration2 = expiration2.to_string();

                let id = user.id.to_string();
                let email = user.email.to_string();

                refresh_claims.insert("sub", "someone");
                refresh_claims.insert("id", &id);
                refresh_claims.insert("email", &email);
                refresh_claims.insert("exp", &expiration2);

                let refresh_token = match refresh_claims.clone().sign_with_key(&refresh_key) {
                    Ok(token) => token,
                    Err(err) => return Err(async_graphql::Error::new("Wrong token".to_string())),
                };

                let mut access_claims = BTreeMap::new();
                access_claims.insert("sub", "someone");
                access_claims.insert("id", &id);
                access_claims.insert("email", &email);
                access_claims.insert("exp", &expiration);
                let access_token = match access_claims.sign_with_key(&access_key) {
                    Ok(token) => token,
                    Err(err) => return Err(async_graphql::Error::new("Wrong token".to_string())),
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
        let key: Hmac<Sha256> = match Hmac::new_from_slice(b"some-secret2") {
            Ok(key) => key,
            Err(err) => return Err(async_graphql::Error::new("Wrong token".to_string())),
        };
        let claims: BTreeMap<String, String> = match access_token.verify_with_key(&key) {
            Ok(res) => res,
            Err(err) => return Err(async_graphql::Error::new("Wrong token".to_string())),
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
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:password@localhost:5432/ss.lv")
            .await
            .expect("error with connection");

    Migrator::up(&db, None).await.expect("migration ban");

    println!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
            .data(Context::new(db.clone())) // add the context here
            .finish();
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:5173")
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
