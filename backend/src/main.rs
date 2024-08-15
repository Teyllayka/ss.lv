mod user_queries;
mod advert_queries;

use user_queries::{UserMutation, UserQuery};
use advert_queries::{AdvertMutation, AdvertQuery};

use std::{collections::BTreeMap, time::{SystemTime, UNIX_EPOCH}};
use actix_cors::Cors;
use dotenvy::dotenv;
use actix_web::{guard, http, web, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptySubscription, MergedObject, Object, Schema, SimpleObject};
use async_graphql_actix_web::GraphQL;
use entity::{
    advert::{self, Entity as Advert},
    user::{self, Entity as User},
};
use hmac::{Hmac, Mac};
use migration::{Migrator, MigratorTrait};
use sea_orm::{
    ColumnTrait, Database, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter,
};
use sha2::Sha256;
use jwt::VerifyWithKey;



pub fn verify_access_token(
    access_token: String,
    access_key: &Hmac<Sha256>,
) -> Result<BTreeMap<String, String>, async_graphql::Error> {
    let claims: BTreeMap<String, String> =
        match access_token.verify_with_key(access_key) {
            Ok(res) => res,
            Err(err) => return Err(async_graphql::Error::new(err.to_string())),
        };
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    if claims["sub"] != "someone" || claims["exp"].parse::<usize>().unwrap() < now {
        return Err(async_graphql::Error::new(
            "you are not logged in".to_string(),
        ));
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

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {

    async fn stats(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Result<Statistics, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

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


// #[derive(Default)]
// pub struct MutationRoot;

// // #[Object]
// // impl MutationRoot {
    
    
// // }

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
    dotenv().expect(".env file not found");
    let db_url = dotenvy::var("DATABASE_URL").expect("HOME environment variable not found");
    let refresh_secret =
        dotenvy::var("REFRESH_SECRET").expect("HOME environment variable not found");
    let access_secret = dotenvy::var("ACCESS_SECRET").expect("HOME environment variable not found");
    let port = (dotenvy::var("BACKEND_PORT").expect("HOME environment variable not found"))
        .parse::<u16>()
        .expect("port is not a number");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    let db: DatabaseConnection = Database::connect(db_url)
        .await
        .expect("error with connection");

    Migrator::up(&db, None).await.expect("migration ban");

    println!("GraphiQL IDE: http://localhost:{}/", port);

    let access_key: Hmac<Sha256> = Hmac::new_from_slice(access_secret.as_bytes()).unwrap();
    let refresh_key: Hmac<Sha256> = Hmac::new_from_slice(refresh_secret.as_bytes()).unwrap();

   

    HttpServer::new(move || {
        let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .data(Context::new(
                db.clone(),
                access_key.clone(),
                refresh_key.clone(),
            ))
            .finish();
        let cors = Cors::default()
            .allow_any_origin()
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
    .bind(("0.0.0.0", port))?
    .run()
    .await
}


