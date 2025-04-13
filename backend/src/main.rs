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
    payment::{self, Entity as Payment},
    user::{self, Entity as User},
};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use migration::{sea_orm::EntityTrait, Migrator, MigratorTrait};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, PaginatorTrait, QueryFilter, Set,
};
use serde_json::Value;
use sha2::Sha256;
use std::{
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};
use stripe::{CheckoutSession, EventObject, EventType, Webhook, WebhookError};
use user_queries::{UserMutation, UserQuery};

pub fn verify_access_token(
    access_token: String,
    access_key: &Hmac<Sha256>,
) -> Result<BTreeMap<String, Value>, Error> {
    // Remove the "Bearer " prefix if present.
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

pub struct Context {
    pub db: DatabaseConnection,
    pub redis_pool: Pool,
    pub stripe_secret: String,
    pub access_key: Hmac<Sha256>,
    pub refresh_key: Hmac<Sha256>,
    pub email_key: Hmac<Sha256>,
    pub username: String,
    pub password: String,
}

impl Context {
    pub fn new(
        db: DatabaseConnection,
        redis_pool: Pool,
        stripe_secret: String,
        access_key: Hmac<Sha256>,
        refresh_key: Hmac<Sha256>,
        email_key: Hmac<Sha256>,
        username: String,
        password: String,
    ) -> Self {
        Self {
            db,
            redis_pool,
            stripe_secret,
            access_key,
            refresh_key,
            email_key,
            username,
            password,
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
    // let mut access_token = String::new();
    // let mut refresh_token  =String::new();
    // match req.cookies() {
    //     Ok(cookies) => {
    //         for cookie in cookies.iter() {
    //             if cookie.name() == "accessToken" {
    //                 access_token = cookie.value().to_string();

    //             } else if cookie.name() == "refreshToken" {
    //                 refresh_token = cookie.value().to_string();
    //             }
    //         }
    //     }
    //     Err(_) => {}
    // }
    // request = request.data(Token(access_token, refresh_token));
    if let Some(token) = get_token_from_headers(req.headers()) {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}

async fn handle_webhook(
    req: HttpRequest,
    payload: web::Bytes,
    ctx: web::Data<Context>,
) -> HttpResponse {
    let payload_str = std::str::from_utf8(&payload).unwrap();

    let stripe_signature = get_header_value(&req, "Stripe-Signature").unwrap_or_default();

    if let Ok(event) = Webhook::construct_event(
        payload_str,
        stripe_signature,
        "whsec_8ae96f7644564cca47b0a534d34590fac5a4df98933620999f25460001c5df2a",
    ) {
        match event.type_ {
            EventType::AccountUpdated => {
                if let EventObject::Account(account) = event.data.object {
                    handle_account_updated(account)
                        .expect("Failed to handle account updated event");
                }
            }
            EventType::CheckoutSessionCompleted => {
                if let EventObject::CheckoutSession(session) = event.data.object {
                    if let Err(err) = handle_checkout_session(session, ctx.clone()).await {
                        println!("Error handling checkout session: {:?}", err);
                        return HttpResponse::InternalServerError().finish();
                    }
                }
            }
            _ => {
                println!("Unknown event encountered in webhook: {:?}", event.type_);
            }
        }
    } else {
        println!("Failed to construct webhook event, ensure your webhook secret is correct.");
    }

    HttpResponse::Ok().finish()
}

fn get_header_value<'b>(req: &'b HttpRequest, key: &'b str) -> Option<&'b str> {
    req.headers().get(key)?.to_str().ok()
}

fn handle_account_updated(account: stripe::Account) -> Result<(), WebhookError> {
    println!(
        "Received account updated webhook for account: {:?}",
        account.id
    );
    Ok(())
}

async fn handle_checkout_session(
    session: CheckoutSession,
    ctx: web::Data<Context>,
) -> Result<(), async_graphql::Error> {
    println!(
        "Received checkout session completed webhook with id: {:?}",
        session.id
    );

    let stripe_session_id = session.payment_link.unwrap();

    let db = &ctx.db;

    let payment = Payment::find()
        .filter(payment::Column::OrderId.eq(&*stripe_session_id.id()))
        .one(db)
        .await
        .expect("Failed to find payment by order id");

    let payment = match payment {
        Some(payment) => payment,
        None => return Err(async_graphql::Error::new("payment not found".to_string())),
    };

    let new_payment: payment::ActiveModel = payment::ActiveModel {
        status: Set(payment::Status::Completed),
        ..payment.into()
    };

    let adv: payment::Model = new_payment.update(db).await?;

    let user = User::find()
        .filter(user::Column::Id.eq(adv.user_id))
        .one(db)
        .await?;

    let user = match user {
        Some(user) => user,
        None => return Err(async_graphql::Error::new("user not found".to_string())),
    };

    let new_user: user::ActiveModel = user::ActiveModel {
        balance: Set(user.balance + adv.amount),
        ..user.into()
    };

    let _: user::Model = new_user.update(db).await?;

    return Ok(());
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
    dotenv().expect(".env file not found");
    let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");
    let redis_url = dotenvy::var("REDIS_URL").expect("REDIS_URL environment variable not found");
    let stripe =
        dotenvy::var("STRIPE_SECRET").expect("STRIPE_SECRET environment variable not found");
    let refresh_secret =
        dotenvy::var("REFRESH_SECRET").expect("REFRESH_SECRET environment variable not found");
    let access_secret =
        dotenvy::var("ACCESS_SECRET").expect("ACCESS_SECRET environment variable not found");
    let username =
        dotenvy::var("MAILJET_USERNAME").expect("MAILJET_USERNAME environment variable not found");
    let password =
        dotenvy::var("MAILJET_PASSWORD").expect("MAILJET_PASSWORD environment variable not found");
    let port = (dotenvy::var("BACKEND_PORT").expect("BACKEND_PORT environment variable not found"))
        .parse::<u16>()
        .expect("port is not a number");
    let ip = dotenvy::var("BACKEND_IP").expect("BACKEND_IP environment variable not found");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    let db: DatabaseConnection = Database::connect(db_url)
        .await
        .expect("error with connection");

    Migrator::up(&db, None).await.expect("Migration error");

    println!("GraphiQL IDE: http://{}:{}/", ip, port);

    let access_key: Hmac<Sha256> = Hmac::new_from_slice(access_secret.as_bytes()).unwrap();
    let refresh_key: Hmac<Sha256> = Hmac::new_from_slice(refresh_secret.as_bytes()).unwrap();
    let email_key: Hmac<Sha256> = Hmac::new_from_slice(username.as_bytes()).unwrap();
    let cfg = Config::from_url(redis_url);
    let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();

    HttpServer::new(move || {
        let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .data(Context::new(
                db.clone(),
                pool.clone(),
                stripe.clone(),
                access_key.clone(),
                refresh_key.clone(),
                email_key.clone(),
                username.clone(),
                password.clone(),
            ))
            .finish();

        let context_data = web::Data::new(Context::new(
            db.clone(),
            pool.clone(),
            stripe.clone(),
            access_key.clone(),
            refresh_key.clone(),
            email_key.clone(),
            username.clone(),
            password.clone(),
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
            .service(
                web::resource("/webhook")
                    .guard(guard::Post())
                    .to(handle_webhook),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind((ip, port))?
    .run()
    .await
}
