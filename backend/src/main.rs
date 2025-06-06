mod advert_queries;
mod user_queries;

use actix_cors::Cors;
use actix_web::HttpRequest;
use actix_web::{
    dev::Service,
    guard,
    http::{
        self,
        header::{self, HeaderMap},
    },
    web, App, HttpResponse, HttpServer, Result,
};
use advert_queries::{AdvertMutation, AdvertQuery};
use async_graphql::Error;
use async_graphql::{
    http::GraphiQLSource, EmptySubscription, MergedObject, Object, Schema, SimpleObject,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use deadpool_redis::{Config, Pool, Runtime};
use dotenvy::dotenv;
use entity::{
    advert::{self, Entity as Advert},
    user::{self, Entity as User},
};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use migration::{sea_orm::EntityTrait, Migrator, MigratorTrait};
use sea_orm::{
    ColumnTrait, Database, DatabaseConnection, PaginatorTrait, QueryFilter,
};
use serde_json::Value;
use sha2::Sha256;
use std::{
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};
use user_queries::{UserMutation, UserQuery};

pub fn verify_access_token(
    access_token: String,
    access_key: &Hmac<Sha256>,
) -> Result<BTreeMap<String, Value>, Error> {
    let access_token = access_token.replace("Bearer ", "");

    let claims: BTreeMap<String, Value> = match access_token.verify_with_key(access_key) {
        Ok(res) => res,
        Err(err) => return Err(Error::new(err.to_string())),
    };

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let sub = claims
        .get("sub")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::new("Missing 'sub' claim"))?;
    if sub != "someone" {
        return Err(Error::new("Invalid subject"));
    }

    let exp = claims
        .get("exp")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| Error::new("Missing or invalid 'exp' claim"))? as usize;

    if exp < now {
        return Err(Error::new("Token expired: you are not logged in"));
    }

    Ok(claims)
}

#[derive(SimpleObject)]
pub struct Statistics {
    pub user_count: u64,
    pub advert_count: u64,
    pub today_user_count: u64,
    pub today_advert_count: u64,
}


#[derive(Debug)]
pub struct Context {
    pub db: DatabaseConnection,
    pub redis_pool: Pool,
    pub access_key: Hmac<Sha256>,
    pub refresh_key: Hmac<Sha256>,
    pub mailersend_token: String,
    pub email_key: Hmac<Sha256>,
}

impl Context {
    pub fn new(
        db: DatabaseConnection,
        redis_pool: Pool,
        access_key: Hmac<Sha256>,
        refresh_key: Hmac<Sha256>,
        mailersend_token: String,
        email_key: Hmac<Sha256>,
    ) -> Self {
        Self {
            db,
            redis_pool,
            // stripe_secret,
            access_key,
            refresh_key,
            mailersend_token,
            email_key,
        }
    }
}

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn stats(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Result<Statistics, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        // let mut conn = my_ctx.redis_pool.get().await.unwrap();
        // cmd("SET")
        // .arg(&["deadpool/test_key", "42"])
        // .query_async::<()>(&mut conn)
        // .await.unwrap();

        let users = User::find();

        let user_count = users.clone().count(&my_ctx.db).await?;

        let adverts = Advert::find();

        let advert_count = adverts.clone().count(&my_ctx.db).await?;

        let today: Option<sea_orm::prelude::DateTime> =
            chrono::Utc::now().naive_utc().date().and_hms_opt(0, 0, 0);

        let today_advert_count = adverts
            .filter(advert::Column::CreatedAt.gt(today))
            .count(&my_ctx.db)
            .await?;

        let today_user_count = users
            .filter(user::Column::CreatedAt.gt(today))
            .count(&my_ctx.db)
            .await?;

        Ok(Statistics {
            user_count,
            advert_count,
            today_user_count,
            today_advert_count,
        })
    }
}

#[derive(Debug)]
pub struct Token(pub String);

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    headers
        .get("authorization")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
}

async fn index(
    schema: web::Data<Schema<Query, Mutation, EmptySubscription>>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_request.into_inner();

    if let Some(token) = get_token_from_headers(req.headers()) {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}


async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}

#[derive(MergedObject, Default)]
struct Query(UserQuery, QueryRoot, AdvertQuery);

#[derive(MergedObject, Default)]
struct Mutation(UserMutation, AdvertMutation);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");
    let redis_url = dotenvy::var("REDIS_URL").expect("REDIS_URL environment variable not found");
    let refresh_secret =
        dotenvy::var("REFRESH_SECRET").expect("REFRESH_SECRET environment variable not found");
    let access_secret =
        dotenvy::var("ACCESS_SECRET").expect("ACCESS_SECRET environment variable not found");
    let port = (dotenvy::var("BACKEND_PORT").expect("BACKEND_PORT environment variable not found"))
        .parse::<u16>()
        .expect("port is not a number");
    let ip = dotenvy::var("BACKEND_IP").expect("BACKEND_IP environment variable not found");
    let mailersend_token = dotenvy::var("MAILERSEND_TOKEN")
        .expect("MAILERSEND_TOKEN environment variable not found");
    let email_key = dotenvy::var("EMAIL_SECRET").expect("EMAIL_SECRET environment variable not found");
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();
    let db: DatabaseConnection = Database::connect(db_url)
        .await
        .expect("error with connection");

    Migrator::up(&db, None).await.expect("Migration error");

    println!("GraphiQL IDE: http://{}:{}/", ip, port);

    let access_key: Hmac<Sha256> = Hmac::new_from_slice(access_secret.as_bytes()).unwrap();
    let refresh_key: Hmac<Sha256> = Hmac::new_from_slice(refresh_secret.as_bytes()).unwrap();
    let email_key: Hmac<Sha256> = Hmac::new_from_slice(email_key.as_bytes()).unwrap();
    let cfg = Config::from_url(redis_url);
    let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();

    HttpServer::new(move || {
        let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .data(Context::new(
                db.clone(),
                pool.clone(),
                // stripe.clone(),
                access_key.clone(),
                refresh_key.clone(),
                mailersend_token.clone(),
                email_key.clone(   )
            ))
            .finish();

        let context_data = web::Data::new(Context::new(
            db.clone(),
            pool.clone(),
            // stripe.clone(),
            access_key.clone(),
            refresh_key.clone(),
            mailersend_token.clone(),
            email_key.clone(),
        ));

        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(schema))
            .app_data(context_data)
            .wrap(cors)
            .wrap_fn(|req, srv| {
                let res = srv.call(req);
                async {
                    let mut response = res.await?;
                    response.headers_mut().insert(
                        header::X_FRAME_OPTIONS,
                        header::HeaderValue::from_static("DENY"),
                    );
                    response.headers_mut().insert(
                        header::X_CONTENT_TYPE_OPTIONS,
                        header::HeaderValue::from_static("nosniff"),
                    );
                    Ok(response)
                }
            })
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind((ip, port))?
    .run()
    .await
}
