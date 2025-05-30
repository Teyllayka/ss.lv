use serde_json::Value;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{verify_access_token, Context, Token};
use actix_web::Result;
use async_graphql::{Object, SimpleObject};
use chrono::Utc;
use deadpool_redis::redis::cmd;
use entity::{
    advert::{self, Entity as Advert},
    chat::{self},
    favorites::{self},
    reviews::{self, Entity as Reviews},
    user::{self, Entity as User},
};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use sea_orm::ColumnTrait;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, Set};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde_json::json;
const ACCESS_EXPIRATION: usize = 100;
const REFRESH_EXPIRATION: usize = 180;

#[derive(SimpleObject)]
#[graphql(name = "LoginResponse")]
pub struct LoginResponse {
    refresh_token: String,
    access_token: String,
    user_id: i32,
}

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn user(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
    ) -> Result<user::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let user = User::find_by_id(id)
            .one(&my_ctx.db)
            .await?
            .ok_or_else(|| async_graphql::Error::new("No user found"))?;

        let adverts_with_review = Advert::find()
            .filter(advert::Column::UserId.eq(user.id))
            .find_also_related(Reviews)
            .all(&my_ctx.db)
            .await?;

        let reviewer_ids: HashSet<i32> = adverts_with_review
            .iter()
            .filter_map(|(_, review_opt)| review_opt.as_ref())
            .map(|review| review.user_id)
            .collect();

        let reviewers = User::find()
            .filter(user::Column::Id.is_in(reviewer_ids.clone()))
            .all(&my_ctx.db)
            .await?;

        let reviewer_map: HashMap<i32, user::Model> =
            reviewers.into_iter().map(|user| (user.id, user)).collect();

        let mut favorite_advert_ids = HashSet::new();

        if let Some(token) = ctx.data_opt::<Token>().map(|token| token.0.clone()) {
            if let Ok(claims) = verify_access_token(token, &my_ctx.access_key) {
                let current_user_id: i32 =
                    if let Some(id_str) = claims.get("id").and_then(|v| v.as_str()) {
                        id_str.parse().map_err(|_| {
                            async_graphql::Error::new(
                                "Invalid user ID in token: failed to parse string",
                            )
                        })?
                    } else if let Some(id_num) = claims.get("id").and_then(|v| v.as_i64()) {
                        id_num as i32
                    } else {
                        return Err(async_graphql::Error::new(
                            "Invalid user ID in token: missing id",
                        ));
                    };

                let favorite_adverts = favorites::Entity::find()
                    .filter(favorites::Column::UserId.eq(current_user_id))
                    .all(&my_ctx.db)
                    .await?;

                favorite_advert_ids = favorite_adverts
                    .into_iter()
                    .map(|fav| fav.advert_id)
                    .collect();
            }
        }

        let mut user_rating: f32 = 0.0;

        let mut adverts = Vec::new();
        let mut adverts_with_reviews = Vec::new();

        for (mut advert, mut review_opt) in adverts_with_review {
            if let Some(review) = review_opt.as_mut() {
                user_rating += review.rating as f32;
                let reviewer = reviewer_map
                    .get(&review.user_id)
                    .cloned()
                    .unwrap_or_else(|| user::Model::default());
                review.user = reviewer;
                advert.review = Some(review.clone());

                adverts_with_reviews.push(advert.clone());
            }

            advert.is_favorited = favorite_advert_ids.contains(&advert.id);
            adverts.push(advert);
        }

        let reviews_written_by_user = Reviews::find()
            .filter(reviews::Column::UserId.eq(user.id))
            .find_also_related(Advert)
            .all(&my_ctx.db)
            .await?;

        let mut reviewed_adverts = Vec::new();

        for (mut review, advert_opt) in reviews_written_by_user {
            if let Some(mut advert) = advert_opt {
                advert.is_favorited = favorite_advert_ids.contains(&advert.id);

                review.user = user.clone();
                advert.review = Some(review);

                reviewed_adverts.push(advert);
            }
        }

        let mut user = user;
        user.adverts = adverts;
        user.adverts_with_reviews = adverts_with_reviews;
        user.reviewed_adverts = reviewed_adverts;
        user.rating = if user_rating > 0.0 && !user.adverts_with_reviews.is_empty() {
            user_rating / (user.adverts_with_reviews.len() as f32)
        } else {
            0.0
        };

        Ok(user)
    }

    async fn me(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Result<user::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone()) {
            Some(token) => token,
            None => {
                return Err(async_graphql::Error::new("You are not logged in."));
            }
        };

        let claims = match verify_access_token(access_token, &my_ctx.access_key) {
            Ok(claims) => claims,
            Err(err) => return Err(err),
        };

        let id: i32 = if let Some(id_str) = claims.get("id").and_then(|v| v.as_str()) {
            id_str.parse().map_err(|_| {
                async_graphql::Error::new("Invalid user ID in token: failed to parse string")
            })?
        } else if let Some(id_num) = claims.get("id").and_then(|v| v.as_i64()) {
            id_num as i32
        } else {
            return Err(async_graphql::Error::new(
                "Invalid user ID in token: missing id",
            ));
        };

        let user = User::find_by_id(id)
            .one(&my_ctx.db)
            .await?
            .ok_or_else(|| async_graphql::Error::new("No user found."))?;

        let adverts_with_review = Advert::find()
            .filter(advert::Column::UserId.eq(user.id))
            .find_also_related(Reviews)
            .all(&my_ctx.db)
            .await?;

        let reviewer_ids: HashSet<i32> = adverts_with_review
            .iter()
            .filter_map(|(_, review_opt)| review_opt.as_ref())
            .map(|review| review.user_id)
            .collect();

        let reviewers = User::find()
            .filter(user::Column::Id.is_in(reviewer_ids.clone()))
            .all(&my_ctx.db)
            .await?;

        let reviewer_map: HashMap<i32, user::Model> =
            reviewers.into_iter().map(|user| (user.id, user)).collect();

        let mut favorite_advert_ids = HashSet::new();

        if let Some(token) = ctx.data_opt::<Token>().map(|token| token.0.clone()) {
            if let Ok(claims) = verify_access_token(token, &my_ctx.access_key) {
                let current_user_id: i32 =
                    if let Some(id_str) = claims.get("id").and_then(|v| v.as_str()) {
                        id_str.parse().map_err(|_| {
                            async_graphql::Error::new(
                                "Invalid user ID in token: failed to parse string",
                            )
                        })?
                    } else if let Some(id_num) = claims.get("id").and_then(|v| v.as_i64()) {
                        id_num as i32
                    } else {
                        return Err(async_graphql::Error::new(
                            "Invalid user ID in token: missing id",
                        ));
                    };

                let favorite_adverts = favorites::Entity::find()
                    .filter(favorites::Column::UserId.eq(current_user_id))
                    .all(&my_ctx.db)
                    .await?;

                favorite_advert_ids = favorite_adverts
                    .into_iter()
                    .map(|fav| fav.advert_id)
                    .collect();
            }
        }

        let mut user_rating: f32 = 0.0;

        let mut adverts = Vec::new();
        let mut adverts_with_reviews = Vec::new();

        for (mut advert, mut review_opt) in adverts_with_review {
            if let Some(review) = review_opt.as_mut() {
                user_rating += review.rating as f32;
                let reviewer = reviewer_map
                    .get(&review.user_id)
                    .cloned()
                    .unwrap_or_else(|| user::Model::default());
                review.user = reviewer;
                advert.review = Some(review.clone());

                adverts_with_reviews.push(advert.clone());
            }

            advert.is_favorited = favorite_advert_ids.contains(&advert.id);
            adverts.push(advert);
        }

        let reviews_written_by_user = Reviews::find()
            .filter(reviews::Column::UserId.eq(user.id))
            .find_also_related(Advert)
            .all(&my_ctx.db)
            .await?;

        let mut reviewed_adverts = Vec::new();

        for (mut review, advert_opt) in reviews_written_by_user {
            if let Some(mut advert) = advert_opt {
                advert.is_favorited = favorite_advert_ids.contains(&advert.id);

                review.user = user.clone();
                advert.review = Some(review);

                reviewed_adverts.push(advert);
            }
        }

        // Update the user model with the fetched data
        let mut user = user;
        user.adverts = adverts;
        user.adverts_with_reviews = adverts_with_reviews;
        user.reviewed_adverts = reviewed_adverts;
        user.rating = if user_rating > 0.0 && !user.adverts_with_reviews.is_empty() {
            user_rating / (user.adverts_with_reviews.len() as f32)
        } else {
            0.0
        };

        Ok(user)
    }

    async fn validate_reset_token(
        &self,
        ctx: &async_graphql::Context<'_>,
        token: String,
    ) -> Result<bool, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();
        let claims: BTreeMap<String, Value> = token
            .verify_with_key(&my_ctx.email_key)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        if claims.get("sub").and_then(|v| v.as_str()) != Some("reset") {
            return Ok(false);
        }
        let exp = claims.get("exp").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        if exp <= now {
            return Ok(false);
        }
        let email = claims
            .get("email")
            .and_then(|v| v.as_str())
            .ok_or_else(|| async_graphql::Error::new("Token missing email"))?;
        let user_exists = User::find_by_email(email.to_string())
            .one(&my_ctx.db)
            .await?
            .is_some();
        Ok(user_exists)
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn register(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(validator(email))] email: String,
        #[graphql(validator(min_length = 8))] password: String,
        surname: Option<String>,
        name: Option<String>,
        company_name: Option<String>,
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

        let user: Option<user::Model> = User::find_by_email(email.clone()).one(&my_ctx.db).await?;

        if user.is_some() {
            return Err(async_graphql::Error::new("User already exists".to_string()));
        }

        let user = user::ActiveModel {
            name: Set(name),
            surname: Set(surname),
            company_name: Set(company_name),
            email: Set(Some(email.clone())),
            password_hash: Set(Some(parsed_hash.to_string())),
            created_at: Set(naive_date_time),
            updated_at: Set(naive_date_time),
            ..Default::default()
        };

        let user: user::Model = user.insert(&my_ctx.db).await?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        let expiration = now + (ACCESS_EXPIRATION * 60);
        let mut email_verif: BTreeMap<&str, Value> = BTreeMap::new();
        email_verif.insert("sub", json!("someone"));
        email_verif.insert(
            "email",
            json!(user.email.as_deref().unwrap_or("default_email")),
        );
        email_verif.insert("exp", json!(expiration));
        let verification = match email_verif.sign_with_key(&my_ctx.email_key) {
            Ok(token) => token,
            Err(err) => return Err(async_graphql::Error::new(err.to_string())),
        };

        println!("{:?}", verification);

        let recipient = user
            .name
            .as_deref()
            .or(user.company_name.as_deref())
            .unwrap_or("User");

        let payload = json!({
            "from": { "email": "info@ad-ee.tech", "name": "Adee" },
            "to": [{ "email": user.email }],
            "subject": "You are awesome!",
            "text": format!(
                "Hi {}, here is your verification link: https://ad-ee.tech/verify_email/{}",
                recipient, verification
            ),
            "html": format!(
                "<p>Hi {},</p><p>Here is your verification link:</p><p><a href=\"https://ad-ee.tech/verify_email/{}\">Verify Email</a></p>",
                recipient, verification
            )
        });

        let client = reqwest::Client::new();
        let resp = client
            .post("https://api.mailersend.com/v1/email")
            .bearer_auth(&my_ctx.mailersend_token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| async_graphql::Error::new(format!("Network error: {}", e)))?;

        if resp.status().is_success() {
            println!("Email sent successfully!");
        } else {
            println!("Failed to send email: {}", resp.text().await?);
        }
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

        let response = if let Some(password_hash) = &user.password_hash {
            argon2
                .verify_password(
                    password.as_bytes(),
                    &PasswordHash::new(password_hash).unwrap(),
                )
                .is_ok()
        } else {
            false
        };

        if !response {
            return Err(async_graphql::Error::new(
                "Wrong email or password".to_string(),
            ));
        }

        let mut refresh_claims: BTreeMap<&str, Value> = BTreeMap::new();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        let expiration = now + (ACCESS_EXPIRATION * 60);
        let expiration2 = now + (REFRESH_EXPIRATION * 60);

        let id = user.id.to_string();
        let email = user.email.expect("Email should not be None").to_string();

        refresh_claims.insert("sub", json!("someone"));
        refresh_claims.insert("id", json!(id));
        refresh_claims.insert("email", json!(email));
        refresh_claims.insert("exp", json!(expiration2));

        let refresh_token = match refresh_claims.clone().sign_with_key(&my_ctx.refresh_key) {
            Ok(token) => token,
            Err(err) => return Err(async_graphql::Error::new(err.to_string())),
        };

        let mut access_claims: BTreeMap<&str, Value> = BTreeMap::new();
        access_claims.insert("sub", json!("someone"));
        access_claims.insert("id", json!(id));
        access_claims.insert("email", json!(email));
        access_claims.insert("exp", json!(expiration));
        let access_token = match access_claims.sign_with_key(&my_ctx.access_key) {
            Ok(token) => token,
            Err(err) => return Err(async_graphql::Error::new(err.to_string())),
        };

        let mut conn = my_ctx.redis_pool.get().await.unwrap();
        cmd("SET")
            .arg(&[
                user.id.to_string(),
                refresh_token.clone(),
                "EX".to_string(),
                expiration2.to_string(),
            ])
            .query_async::<()>(&mut conn)
            .await
            .unwrap();

        ctx.append_http_header("Set-Cookie", format!("refreshToken={}", refresh_token));
        ctx.append_http_header("Set-Cookie", format!("accessToken={}", access_token));
        ctx.append_http_header("Set-Cookie", format!("userId={}", user.id));

        Ok(LoginResponse {
            refresh_token,
            access_token,
            user_id: user.id,
        })
    }

    async fn edit(
        &self,
        ctx: &async_graphql::Context<'_>,
        name: Option<String>,
        surname: Option<String>,
        company_name: Option<String>,
        phone: Option<String>,
        avatar_url: Option<String>,
        password: String,
    ) -> Result<user::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        println!("avatar url: {:?}", avatar_url);

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone()) {
            Some(token) => token.split(' ').collect::<Vec<&str>>()[1].to_string(),
            None => {
                return Err(async_graphql::Error::new(
                    "you are not logged in".to_string(),
                ));
            }
        };

        let claims = match verify_access_token(access_token, &my_ctx.access_key) {
            Ok(claims) => claims,
            Err(err) => return Err(err),
        };

        let id: i32 = if let Some(id_str) = claims.get("id").and_then(|v| v.as_str()) {
            id_str.parse().map_err(|_| {
                async_graphql::Error::new("Invalid user ID in token: failed to parse string")
            })?
        } else if let Some(id_num) = claims.get("id").and_then(|v| v.as_i64()) {
            id_num as i32
        } else {
            return Err(async_graphql::Error::new(
                "Invalid user ID in token: missing id",
            ));
        };
        let user: Option<user::Model> = User::find_by_id(id).one(&my_ctx.db).await?;

        let user = match user {
            Some(user) => user,
            None => return Err(async_graphql::Error::new("Wrong token".to_string())),
        };

        let argon2 = Argon2::default();

        let response = if let Some(password_hash) = &user.password_hash {
            argon2
                .verify_password(
                    password.as_bytes(),
                    &PasswordHash::new(password_hash).unwrap(),
                )
                .is_ok()
        } else {
            false
        };

        if !response {
            return Err(async_graphql::Error::new("Wrong password".to_string()));
        }

        let mut active_user = user::ActiveModel {
            id: Set(user.id),
            ..user.into()
        };

        if let Some(name) = name {
            active_user.name = Set(Some(name));
        }
        if let Some(surname) = surname {
            active_user.surname = Set(Some(surname));
        }
        if let Some(company_name) = company_name {
            active_user.company_name = Set(Some(company_name));
        }
        if let Some(phone) = phone {
            active_user.phone = Set(Some(phone));
        }
        if let Some(avatar_url) = avatar_url {
            active_user.avatar_url = Set(Some(avatar_url));
        }

        let updated_user: user::Model = active_user.update(&my_ctx.db).await?;

        Ok(updated_user)
    }

    async fn refresh(
        &self,
        ctx: &async_graphql::Context<'_>,
        refresh_token: String,
    ) -> Result<LoginResponse, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let claims: BTreeMap<String, Value> =
            match refresh_token.verify_with_key(&my_ctx.refresh_key) {
                Ok(res) => res,
                Err(err) => {
                    return Err(async_graphql::Error::new(err.to_string()));
                }
            };

        println!("{:?}", claims);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u128;

        if claims["sub"].as_str().expect("Err as str") != "someone"
            || claims["exp"]
                .as_number()
                .expect("Err as number")
                .as_u128()
                .expect("Err as u128")
                < now
        {
            return Err(async_graphql::Error::new("Wrong token".to_string()));
        }

        let id = claims["id"]
            .as_str()
            .expect("Wrong token")
            .parse::<i32>()
            .unwrap();

        let user: Option<user::Model> = User::find_by_id(id).one(&my_ctx.db).await?;

        let user = match user {
            Some(user) => user,
            None => return Err(async_graphql::Error::new("Wrong token".to_string())),
        };

        let mut conn = my_ctx.redis_pool.get().await.unwrap();
        let token: String = cmd("GET")
            .arg(&[user.id.to_string()])
            .query_async(&mut conn)
            .await
            .unwrap();

        if refresh_token != token {
            return Err(async_graphql::Error::new("Wrong token".to_string()));
        }

        let mut refresh_claims: BTreeMap<&str, Value> = BTreeMap::new();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        let expiration = now + (ACCESS_EXPIRATION * 60); // 1 minutes from now
        let expiration2 = now + (REFRESH_EXPIRATION * 60); // 60 minutes from now

        let id = user.id.to_string();
        let email = user.email.expect("Email should not be None").to_string();

        refresh_claims.insert("sub", json!("someone"));
        refresh_claims.insert("id", json!(id));
        refresh_claims.insert("email", json!(email));
        refresh_claims.insert("exp", json!(expiration2));

        let refresh_token = match refresh_claims.clone().sign_with_key(&my_ctx.refresh_key) {
            Ok(token) => token,
            Err(err) => return Err(async_graphql::Error::new(err.to_string())),
        };

        let mut access_claims: BTreeMap<&str, Value> = BTreeMap::new();
        access_claims.insert("sub", json!("someone"));
        access_claims.insert("id", json!(id));
        access_claims.insert("email", json!(email));
        access_claims.insert("exp", json!(expiration));
        let access_token = match access_claims.sign_with_key(&my_ctx.access_key) {
            Ok(token) => token,
            Err(err) => return Err(async_graphql::Error::new(err.to_string())),
        };

        cmd("SET")
            .arg(&[
                user.id.to_string(),
                refresh_token.clone(),
                "EX".to_string(),
                expiration2.to_string(),
            ])
            .query_async::<()>(&mut conn)
            .await
            .unwrap();

        return Ok(LoginResponse {
            refresh_token,
            access_token,
            user_id: user.id,
        });
    }

    async fn verify_email(
        &self,
        ctx: &async_graphql::Context<'_>,
        token: String,
    ) -> Result<String, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let claims: BTreeMap<String, Value> = match token.verify_with_key(&my_ctx.email_key) {
            Ok(res) => res,
            Err(err) => return Err(async_graphql::Error::new(err.to_string())),
        };

        println!("{:?}", claims);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u128;

        if claims["sub"].as_str().expect("Err as str") != "someone"
            || claims["exp"]
                .as_number()
                .expect("Err as number")
                .as_u128()
                .expect("Err as u128")
                < now
        {
            return Err(async_graphql::Error::new("wrong token".to_string()));
        }

        let email = claims["email"]
            .as_str()
            .expect("email should be a string")
            .to_string();

        let user: Option<user::Model> = User::find_by_email(email).one(&my_ctx.db).await?;

        let user = match user {
            Some(user) => user,
            None => return Err(async_graphql::Error::new("Wrong token".to_string())),
        };

        if user.email_verified {
            return Err(async_graphql::Error::new(
                "Email already verified".to_string(),
            ));
        }

        user::ActiveModel {
            id: Set(user.id),
            email_verified: Set(true),
            ..Default::default()
        }
        .update(&my_ctx.db)
        .await?;

        return Ok("Email verified".to_string());
    }

    async fn resend_email(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Result<String, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone()) {
            Some(token) => token.split(' ').collect::<Vec<&str>>()[1].to_string(),
            None => {
                return Err(async_graphql::Error::new(
                    "you are not logged in".to_string(),
                ));
            }
        };

        let claims = match verify_access_token(access_token, &my_ctx.access_key) {
            Ok(claims) => claims,
            Err(err) => return Err(err),
        };

        let id: i32 = if let Some(id_str) = claims.get("id").and_then(|v| v.as_str()) {
            id_str.parse().map_err(|_| {
                async_graphql::Error::new("Invalid user ID in token: failed to parse string")
            })?
        } else if let Some(id_num) = claims.get("id").and_then(|v| v.as_i64()) {
            id_num as i32
        } else {
            return Err(async_graphql::Error::new(
                "Invalid user ID in token: missing id",
            ));
        };
        let user: Option<user::Model> = User::find_by_id(id).one(&my_ctx.db).await?;

        let user = match user {
            Some(user) => user,
            None => return Err(async_graphql::Error::new("Wrong token".to_string())),
        };

        if user.email_verified {
            return Err(async_graphql::Error::new(
                "Email already verified".to_string(),
            ));
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        let expiration = now + (ACCESS_EXPIRATION * 60); // 1 minutes from now
        let mut email_verif: BTreeMap<&str, Value> = BTreeMap::new();
        email_verif.insert("sub", json!("someone"));
        email_verif.insert(
            "email",
            json!(user.email.as_deref().unwrap_or("default_email")),
        );
        email_verif.insert("exp", json!(expiration));
        let verification = match email_verif.sign_with_key(&my_ctx.email_key) {
            Ok(token) => token,
            Err(err) => return Err(async_graphql::Error::new(err.to_string())),
        };

        println!("{:?}", verification);

        let recipient = user
            .name
            .as_deref()
            .or(user.company_name.as_deref())
            .unwrap_or("User");

        let payload = json!({
            "from": { "email": "info@ad-ee.tech", "name": "Adee" },
            "to": [{ "email": user.email }],
            "subject": "You are awesome!",
            "text": format!(
                "Hi {}, here is your verification link: https://ad-ee.tech/verify_email/{}",
                recipient, verification
            ),
            "html": format!(
                "<p>Hi {},</p><p>Here is your verification link:</p><p><a href=\"https://ad-ee.tech/verify_email/{}\">Verify Email</a></p>",
                recipient, verification
            )
        });

        let client = reqwest::Client::new();
        let resp = client
            .post("https://api.mailersend.com/v1/email")
            .bearer_auth(&my_ctx.mailersend_token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| async_graphql::Error::new(format!("Network error: {}", e)))?;

        if resp.status().is_success() {
            println!("Email sent successfully!");
        } else {
            println!("Failed to send email: {}", resp.text().await?);
        }

        return Ok("Email sent".to_string());
    }

    async fn ban_user(
        &self,
        ctx: &async_graphql::Context<'_>,
        user_id: i32,
    ) -> Result<user::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone()) {
            Some(token) => token.split(' ').collect::<Vec<&str>>()[1].to_string(),
            None => {
                return Err(async_graphql::Error::new(
                    "you are not logged in".to_string(),
                ));
            }
        };

        let claims = match verify_access_token(access_token, &my_ctx.access_key) {
            Ok(claims) => claims,
            Err(err) => return Err(err),
        };

        let id: i32 = if let Some(id_str) = claims.get("id").and_then(|v| v.as_str()) {
            id_str.parse().map_err(|_| {
                async_graphql::Error::new("Invalid user ID in token: failed to parse string")
            })?
        } else if let Some(id_num) = claims.get("id").and_then(|v| v.as_i64()) {
            id_num as i32
        } else {
            return Err(async_graphql::Error::new(
                "Invalid user ID in token: missing id",
            ));
        };
        let caller: Option<user::Model> = User::find_by_id(id).one(&my_ctx.db).await?;

        match caller {
            Some(u) => {
                if u.role == user::Role::Admin {
                    ()
                } else {
                    return Err(async_graphql::Error::new(
                        "You are not authorized to ban users".to_string(),
                    ));
                }
            }
            None => return Err(async_graphql::Error::new("Wrong token".to_string())),
        };

        let target = user::Entity::find_by_id(user_id)
            .one(&my_ctx.db)
            .await?
            .ok_or_else(|| async_graphql::Error::new("User not found"))?;

        let active_user = user::ActiveModel {
            id: Set(user_id),
            banned: Set(!target.banned),
            ..Default::default()
        };

        let updated_user: user::Model = active_user.update(&my_ctx.db).await?;

        let deletion_result = advert::Entity::delete_many()
            .filter(advert::Column::UserId.eq(user_id))
            .exec(&my_ctx.db)
            .await?;

        println!("Deleted {:?} adverts", deletion_result);

        let chat_delete_result = chat::Entity::delete_many()
            .filter(chat::Column::ParticipantId.eq(user_id))
            .exec(&my_ctx.db)
            .await?;

        println!("Deleted {:?} chats", chat_delete_result);

        return Ok(updated_user);
    }

    async fn forgot_password(
        &self,
        ctx: &async_graphql::Context<'_>,
        email: String,
    ) -> Result<String, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();
        let user_opt = User::find_by_email(email.clone()).one(&my_ctx.db).await?;
        let user =
            user_opt.ok_or_else(|| async_graphql::Error::new("No user found with that email"))?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        let exp = now + 60 * 60;
        let mut claims = BTreeMap::new();
        claims.insert("sub", json!("reset"));
        claims.insert("email", json!(email));
        claims.insert("exp", json!(exp));
        let token = claims
            .sign_with_key(&my_ctx.email_key)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let reset_link = format!("https://ad-ee.tech/reset/{}", token);
        let payload = json!({
            "from": { "email": "info@ad-ee.tech", "name": "Adee" },
            "to": [{ "email": user.email.clone().unwrap() }],
            "subject": "Reset your password",
            "text": format!("Click here to reset your password: {}", reset_link),
            "html": format!("<p>Click <a href=\"{}\">here</a> to reset your password.</p>", reset_link)
        });
        let client = reqwest::Client::new();
        client
            .post("https://api.mailersend.com/v1/email")
            .bearer_auth(&my_ctx.mailersend_token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| async_graphql::Error::new(format!("Email send error: {}", e)))?;

        Ok("Password reset email sent".to_string())
    }

    async fn reset_password(
        &self,
        ctx: &async_graphql::Context<'_>,
        token: String,
        new_password: String,
    ) -> Result<String, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();
        let claims: BTreeMap<String, serde_json::Value> = token
            .verify_with_key(&my_ctx.email_key)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        if claims.get("sub").and_then(|v| v.as_str()) != Some("reset") {
            return Err(async_graphql::Error::new("Invalid reset token"));
        }
        let exp = claims.get("exp").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        if exp < now {
            return Err(async_graphql::Error::new("Reset token has expired"));
        }
        let email = claims
            .get("email")
            .and_then(|v| v.as_str())
            .ok_or_else(|| async_graphql::Error::new("Token missing email"))?;
        let user_opt = User::find_by_email(email.to_string())
            .one(&my_ctx.db)
            .await?;
        let user = user_opt.ok_or_else(|| async_graphql::Error::new("User not found"))?;

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        let hash_string = hash.to_string();
        let parsed = PasswordHash::new(&hash_string)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let now_naive = Utc::now().naive_utc();
        let mut active: user::ActiveModel = user.into();
        active.password_hash = Set(Some(parsed.to_string()));
        active.updated_at = Set(now_naive);
        active.update(&my_ctx.db).await?;

        Ok("Password has been reset successfully".to_string())
    }
}
