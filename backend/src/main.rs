use actix_cors::Cors;
use dotenvy::dotenv;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    time::{SystemTime, UNIX_EPOCH},
};

use actix_web::{guard, http, web, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Json, Object, Schema, SimpleObject};
use async_graphql_actix_web::GraphQL;
use chrono::Utc;
use entity::{
    advert::{self, Entity as Advert},
    favorites::{self, Entity as Favorites},
    specifications::{self, Entity as Specifications},
    user::{self, Entity as User},
};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use migration::{Migrator, MigratorTrait};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, DbErr, DeleteResult, EntityTrait,
    ModelTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use sha2::Sha256;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(SimpleObject)]
struct AdvertWithUser {
    advert: advert::Model,
    user: user::Model,
    is_admin: bool,
    belongs_to_user: bool,
}

const ACCESS_EXPIRATION: usize = 100;
const REFRESH_EXPIRATION: usize = 180;

#[derive(SimpleObject)]
#[graphql(name = "LoginResponse")]
pub struct LoginResponse {
    refresh_token: String,
    access_token: String,
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

    async fn get_advert(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
        access_token: String,
    ) -> Result<AdvertWithUser, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let advert: Option<advert::Model> = Advert::find_by_id(id).one(&my_ctx.db).await?;

        let mut advert = match advert {
            Some(advert) => advert,
            None => return Err(async_graphql::Error::new("advert not found".to_string())),
        };

        let specs: Vec<specifications::Model> =
            advert.find_related(Specifications).all(&my_ctx.db).await?;

        advert.specs = specs;

        let user: Option<user::Model> = User::find_by_id(advert.user_id).one(&my_ctx.db).await?;

        let user = match user {
            Some(user) => user,
            None => return Err(async_graphql::Error::new("user not found".to_string())),
        };

        if access_token.is_empty() {
            return Ok(AdvertWithUser {
                advert,
                user,
                is_admin: false,
                belongs_to_user: false,
            });
        }

        let claims: BTreeMap<String, String> =
            match access_token.verify_with_key(&my_ctx.access_key) {
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

        let user_id: i32 = claims["id"].parse().unwrap();
        let req_user: Option<user::Model> = User::find_by_id(user_id).one(&my_ctx.db).await?;

        let req_user = match req_user {
            Some(req_user) => req_user,
            None => return Err(async_graphql::Error::new("Wrong token".to_string())),
        };

        let favorite_adverts: Vec<favorites::Model> =
            req_user.find_related(Favorites).all(&my_ctx.db).await?;

        let is_admin = req_user.is_admin;

        let mut favorite_adverts_map: HashMap<i32, bool> = HashMap::new();
        for favorite in favorite_adverts {
            favorite_adverts_map.insert(favorite.advert_id, true);
        }

        let is_favorited = favorite_adverts_map.get(&advert.id).is_some();

        let belongs_to_user = user.id == advert.user_id;

        return Ok(AdvertWithUser {
            advert: advert::Model {
                is_favorited,
                ..advert
            },
            user,
            is_admin,
            belongs_to_user,
        });
    }

    async fn get_adverts(
        &self,
        ctx: &async_graphql::Context<'_>,
        access_token: String,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<advert::Model>, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();
        let mut adverts: Vec<advert::Model> = advert::Entity::find()
            .order_by(advert::Column::Id, Order::Desc)
            .offset(offset as u64)
            .limit(limit as u64)
            .filter(advert::Column::Available.eq(true))
            .all(&my_ctx.db)
            .await?;

        for advert in &mut adverts {
            let specs: Vec<specifications::Model> =
                advert.find_related(Specifications).all(&my_ctx.db).await?;
            advert.specs = specs;
        }
        if access_token.is_empty() {
            return Ok(adverts);
        }
        for advert in &mut adverts {
            let specs: Vec<specifications::Model> =
                advert.find_related(Specifications).all(&my_ctx.db).await?;
            advert.specs = specs;
        }

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
            let id: i32 = claims["id"].parse().unwrap();
            let user: Option<user::Model> = User::find_by_id(id).one(&my_ctx.db).await?;

            if let Some(user) = user {
                let favorite_adverts = favorites::Entity::find()
                    .filter(favorites::Column::UserId.eq(user.id))
                    .all(&my_ctx.db)
                    .await?;

                let favorite_advert_ids: HashSet<i32> =
                    favorite_adverts.iter().map(|fav| fav.advert_id).collect();

                for advert in &mut adverts {
                    if favorite_advert_ids.contains(&advert.id) {
                        advert.is_favorited = true;
                    }
                }
            } else {
                return Err(async_graphql::Error::new("Wrong token".to_string()));
            }
        }

        return Ok(adverts);
    }

    async fn get_adverts_by_category(
        &self,
        ctx: &async_graphql::Context<'_>,
        category: String,
    ) -> Result<Vec<advert::Model>, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let adverts: Option<Vec<advert::Model>> = Some(
            Advert::find()
                .filter(advert::Column::Category.eq(category))
                .all(&my_ctx.db)
                .await?,
        );

        let adverts = match adverts {
            Some(advert) => advert,
            None => return Err(async_graphql::Error::new("advert not found".to_string())),
        };

        return Ok(adverts);
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

        if claims["sub"] != "someone" || claims["exp"].parse::<usize>().unwrap() < now {
            return Err(async_graphql::Error::new(
                "you are not logged in".to_string(),
            ));
        }

        let id: i32 = claims["id"].parse().unwrap();
        let user: Option<user::Model> = User::find_by_id(id).one(&my_ctx.db).await?;

        let mut user = match user {
            Some(user) => user,
            None => return Err(async_graphql::Error::new("Wrong token".to_string())),
        };

        let adverts: Vec<advert::Model> = user.find_related(Advert).all(&my_ctx.db).await?;

        user.adverts = adverts;

        return Ok(user);
    }

    async fn get_favorites(
        &self,
        ctx: &async_graphql::Context<'_>,
        access_token: String,
    ) -> Result<Vec<advert::Model>, async_graphql::Error> {
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

        if claims["sub"] != "someone" || claims["exp"].parse::<usize>().unwrap() < now {
            return Err(async_graphql::Error::new(
                "you are not logged in".to_string(),
            ));
        }

        let id: i32 = claims["id"].parse().unwrap();
        let user: Option<user::Model> = User::find_by_id(id).one(&my_ctx.db).await?;

        let user = match user {
            Some(user) => user,
            None => return Err(async_graphql::Error::new("Wrong token".to_string())),
        };

        let favorites: Vec<favorites::Model> = user.find_related(Favorites).all(&my_ctx.db).await?;

        let ids: Vec<i32> = favorites
            .iter()
            .map(|favorite| favorite.advert_id)
            .collect();

        let adverts: Option<Vec<advert::Model>> = Some(
            Advert::find()
                .filter(advert::Column::Id.is_in(ids))
                .all(&my_ctx.db)
                .await?,
        );

        let adverts = match adverts {
            Some(adverts) => adverts,
            None => return Err(async_graphql::Error::new("There is no adverts".to_string())),
        };

        let mut all_adverts: Vec<advert::Model> = Vec::new();
        for advert in adverts {
            all_adverts.push(advert::Model {
                is_favorited: true,
                ..advert
            });
        }

        return Ok(all_adverts);
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
        image: String,
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
            avatar_url: Set(image),
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

        if !response {
            return Err(async_graphql::Error::new(
                "Wrong email or password".to_string(),
            ));
        }

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
    }

    async fn edit_advert(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
        access_token: String,
        price: f32,
        location: String,
        title: String,
        description: String,
        photos: Vec<String>,
        // data: Json<serde_json::Value>,
    ) -> Result<advert::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        if access_token.is_empty() {
            return Err(async_graphql::Error::new(
                "you are not logged in".to_string(),
            ));
        };

        let claims: BTreeMap<String, String> =
            match access_token.verify_with_key(&my_ctx.access_key) {
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

        let user_id: i32 = claims["id"].parse().unwrap();

        let advert: Option<advert::Model> = Advert::find_by_id(id).one(&my_ctx.db).await?;

        let advert = match advert {
            Some(advert) => advert,
            None => return Err(async_graphql::Error::new("advert not found".to_string())),
        };

        if advert.user_id != user_id {
            return Err(async_graphql::Error::new("you are not owner".to_string()));
        }

        // let specs: Vec<specifications::Model> =
        //     advert.find_related(Specifications).all(&my_ctx.db).await?;

        // let features = data.as_object();

        // match features {
        //     Some(features) => {
        //         for (key, value) in features.iter() {
        //             for spec in &specs {
        //                 if spec.key == *key && spec.value != value.as_str().unwrap() {
        //                     let new_spec = specifications::ActiveModel {
        //                         value: Set(value.as_str().unwrap().to_string()),
        //                         ..spec.clone().into()
        //                     };

        //                     new_spec.update(&my_ctx.db).await?;
        //                 }
        //             }
        //         }
        //     }
        //     _ => (),
        // }

        let photo_url: String;
        let additional_photos: Vec<String>;

        if !photos[0].is_empty() {
            photo_url = photos[0].clone();
            additional_photos = photos[1..].iter().cloned().collect();
        } else {
            photo_url = advert.photo_url.clone();
            additional_photos = advert.additional_photos.clone();
        }

        let new_advert = advert::ActiveModel {
            photo_url: Set(photo_url),
            additional_photos: Set(additional_photos),
            price: Set(price),
            location: Set(location),
            title: Set(title),
            description: Set(description),
            updated_at: Set(Utc::now().naive_utc()),
            ..advert.into()
        };

        let adv: advert::Model = new_advert.update(&my_ctx.db).await?;

        return Ok(adv);
    }

    async fn edit(
        &self,
        ctx: &async_graphql::Context<'_>,
        access_token: String,
        name: String,
        surname: String,
        phone: String,
        avatar_url: String,
        password: String,
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

        if claims["sub"] != "someone" || claims["exp"].parse::<usize>().unwrap() < now {
            return Err(async_graphql::Error::new(
                "you are not logged in".to_string(),
            ));
        }

        let id: i32 = claims["id"].parse().unwrap();
        let user: Option<user::Model> = User::find_by_id(id).one(&my_ctx.db).await?;

        let user = match user {
            Some(user) => user,
            None => return Err(async_graphql::Error::new("Wrong token".to_string())),
        };

        let argon2 = Argon2::default();

        let response = argon2
            .verify_password(
                password.as_bytes(),
                &PasswordHash::new(&user.password_hash).unwrap(),
            )
            .is_ok();

        if !response {
            return Err(async_graphql::Error::new(
                "Wrong email or password".to_string(),
            ));
        }

        let mut user = user::ActiveModel {
            name: Set(name),
            surname: Set(surname),
            phone: Set(phone),
            ..user.into()
        };

        if !avatar_url.is_empty() {
            user.avatar_url = Set(avatar_url);
        }

        let user: user::Model = user.update(&my_ctx.db).await?;

        return Ok(user);
    }

    async fn add_favorite(
        &self,
        ctx: &async_graphql::Context<'_>,
        access_token: String,
        advert_id: i32,
    ) -> Result<favorites::Model, async_graphql::Error> {
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

        if claims["sub"] != "someone" || claims["exp"].parse::<usize>().unwrap() < now {
            return Err(async_graphql::Error::new(
                "you are not logged in".to_string(),
            ));
        }

        let id: i32 = claims["id"].parse().unwrap();
        let user: Option<user::Model> = User::find_by_id(id).one(&my_ctx.db).await?;

        let user = match user {
            Some(user) => user,
            None => return Err(async_graphql::Error::new("Wrong token".to_string())),
        };

        let favorite = favorites::ActiveModel {
            advert_id: Set(advert_id),
            user_id: Set(user.id),
            string: Set(format!("{}-{}", user.id, advert_id)),
            created_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        let favorite: favorites::Model = favorite.insert(&my_ctx.db).await?;

        return Ok(favorite);
    }

    async fn remove_favorite(
        &self,
        ctx: &async_graphql::Context<'_>,
        access_token: String,
        advert_id: i32,
    ) -> Result<favorites::Model, async_graphql::Error> {
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

        if claims["sub"] != "someone" || claims["exp"].parse::<usize>().unwrap() < now {
            return Err(async_graphql::Error::new(
                "you are not logged in".to_string(),
            ));
        }

        let id: i32 = claims["id"].parse().unwrap();

        let favorite_result: Result<Option<favorites::Model>, DbErr> = Favorites::find()
            .filter(favorites::Column::AdvertId.eq(advert_id))
            .filter(favorites::Column::UserId.eq(id))
            .one(&my_ctx.db)
            .await;

        let favorite = match favorite_result {
            Ok(favorite_option) => match favorite_option {
                Some(favorite) => favorite,
                None => return Err(async_graphql::Error::new("Wrong favorite".to_string())),
            },
            Err(err) => return Err(async_graphql::Error::new(err.to_string())),
        };

        let _: DeleteResult = favorite.clone().delete(&my_ctx.db).await?;

        return Ok(favorite);
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

        if claims["sub"] != "someone" || claims["exp"].parse::<usize>().unwrap() < now {
            return Err(async_graphql::Error::new("Wrong token".to_string()));
        }

        let id = claims["id"].parse::<i32>().unwrap();

        let user: Option<user::Model> = User::find_by_id(id).one(&my_ctx.db).await?;

        let user = match user {
            Some(user) => user,
            None => return Err(async_graphql::Error::new("Wrong token".to_string())),
        };

        if user.refresh_token != Some(refresh_token.clone()) {
            return Err(async_graphql::Error::new("Wrong token".to_string()));
        }

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

        return Ok(LoginResponse {
            refresh_token,
            access_token,
        });
    }

    async fn create_advert(
        &self,
        ctx: &async_graphql::Context<'_>,
        access_token: String,
        price: f32,
        location: String,
        title: String,
        description: String,
        category: String,
        photos: Vec<String>,
        data: Json<serde_json::Value>,
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

        if claims["sub"] != "someone" || claims["exp"].parse::<usize>().unwrap() < now {
            return Err(async_graphql::Error::new("Wrong token".to_string()));
        }

        let naive_date_time = Utc::now().naive_utc();

        let photo_url = photos[0].clone();

        let additional_photos: Vec<String> = photos[1..].iter().cloned().collect();

        let advert = advert::ActiveModel {
            available: Set(true),
            user_id: Set(claims["id"].parse().unwrap()),
            created_at: Set(naive_date_time),
            updated_at: Set(naive_date_time),
            price: Set(price),
            location: Set(location),
            description: Set(description),
            title: Set(title),
            category: Set(category),
            photo_url: Set(photo_url),
            additional_photos: Set(additional_photos),
            ..Default::default()
        };

        let advert: advert::Model = advert.insert(&my_ctx.db).await?;

        let features = data.as_object();

        match features {
            Some(features) => {
                let mut specifications = Vec::new();
                for (key, value) in features.iter() {
                    let spec = specifications::ActiveModel {
                        key: Set(key.clone()),
                        value: Set(value.as_str().unwrap().to_string()),
                        advert_id: Set(advert.id),
                        ..Default::default()
                    };

                    specifications.push(spec);
                }

                match Specifications::insert_many(specifications)
                    .exec(&my_ctx.db)
                    .await
                {
                    Ok(inserted_specifications) => {
                        println!("{:?}", inserted_specifications);
                    }
                    Err(e) => {
                        eprintln!("Failed to insert specifications: {}", e);
                    }
                }
            }
            _ => (),
        }

        return Ok(advert);
    }

    async fn delete_advert(
        &self,
        ctx: &async_graphql::Context<'_>,
        access_token: String,
        advert_id: i32,
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

        if claims["sub"] != "someone" || claims["exp"].parse::<usize>().unwrap() < now {
            return Err(async_graphql::Error::new("Wrong token".to_string()));
        }

        let user_id = claims["id"].parse::<i32>().unwrap();

        let req_user: Option<user::Model> = User::find_by_id(user_id).one(&my_ctx.db).await?;

        match req_user {
            Some(req_user) => match req_user.is_admin {
                true => (),
                false => {
                    return Err(async_graphql::Error::new(
                        "You dont have rights to do it".to_string(),
                    ))
                }
            },
            None => return Err(async_graphql::Error::new("Wrong token".to_string())),
        };

        let advert: Option<advert::Model> = Advert::find_by_id(advert_id).one(&my_ctx.db).await?;

        let advert = match advert {
            Some(advert) => advert,
            None => return Err(async_graphql::Error::new("advert not found".to_string())),
        };

        advert.clone().delete(&my_ctx.db).await?;

        return Ok(advert);
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
        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
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


