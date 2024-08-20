use std::{
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{verify_access_token, Context, Token};
use sea_orm::{
    ActiveModelTrait, EntityTrait,
    ModelTrait, Set,
};

use actix_web::Result;
use async_graphql::{Object, SimpleObject};
use chrono::Utc;
use entity::{
    advert::{self, Entity as Advert},
    user::{self, Entity as User},
};
use jwt::SignWithKey;
use jwt::VerifyWithKey;

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

        let user: Option<user::Model> = User::find_by_id(id).one(&my_ctx.db).await?;

        let mut user = match user {
            Some(user) => user,
            None => {
                return Err(async_graphql::Error::new(
                    "No user found".to_string(),
                ))
            }
        };

        let adverts: Vec<advert::Model> = user.find_related(Advert).all(&my_ctx.db).await?;

        user.adverts = adverts;

        return Ok(user);
    }

    async fn me(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Result<user::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone())  {
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
}


#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn register(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(validator(email))] email: String,
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


        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        let expiration = now + (ACCESS_EXPIRATION * 60); // 1 minutes from now
        let expiration = expiration.to_string();
        let mut email_verif = BTreeMap::new();
        email_verif.insert("sub", "someone");
        email_verif.insert("email", &user.email);
        email_verif.insert("exp", &expiration);
        let verification = match email_verif.sign_with_key(&my_ctx.email_key) {
            Ok(token) => token,
            Err(err) => return Err(async_graphql::Error::new(err.to_string())),
        };

        
        let payload = json!({
            "Messages": [
                {
                    "From": {
                        "Email": &user.email,
                        "Name": "Adee"
                    },
                    "To": [
                        {
                            "Email": "teyylayt@gmail.com",
                            "Name": "You"
                        }
                    ],
                    "Subject": "My first Mailjet Email!",
                    "TextPart": "Greetings from Mailjet!",
                    "HTMLPart": format!("<h3>Dear passenger 1, welcome to <a href=\"http://localhost:5173/verify_email/\">Mailjet</a>!</h3><br />May the delivery force be with you! {:?}", verification)
                }
            ]
        }
        );

        

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.mailjet.com/v3.1/send")
            .header("Accept", "application/json").basic_auth(my_ctx.username.clone(), Some(my_ctx.password.clone()))
            .json(&payload)
            .send()
            .await?;

        println!("{:?}", response);

        // Handle the response here


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


        ctx.append_http_header("Set-Cookie", format!("refresh_token={}", refresh_token));
        ctx.append_http_header("Set-Cookie", format!("access_token={}", access_token));


        Ok(LoginResponse {
            refresh_token,
            access_token,
        })
    }


    async fn edit(
        &self,
        ctx: &async_graphql::Context<'_>,
        name: String,
        surname: String,
        phone: String,
        avatar_url: String,
        password: String,
    ) -> Result<user::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone())  {
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



    async fn verify_email(
        &self,
        ctx: &async_graphql::Context<'_>,
        token: String,
    ) -> Result<String, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let claims: BTreeMap<String, String> =
            match token.verify_with_key(&my_ctx.email_key) {
                Ok(res) => res,
                Err(err) => return Err(async_graphql::Error::new(err.to_string())),
            };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        if claims["sub"] != "someone" || claims["exp"].parse::<usize>().unwrap() < now {
            return Err(async_graphql::Error::new(
                "wrong token".to_string(),
            ));
        }

        let email = claims["email"].clone();

        let user: Option<user::Model> = User::find_by_email(email).one(&my_ctx.db).await?;

        let user = match user {
            Some(user) => user,
            None => return Err(async_graphql::Error::new("Wrong token".to_string())),
        };

        if user.email_verified {
            return Err(async_graphql::Error::new("Email already verified".to_string()));
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

    
}
