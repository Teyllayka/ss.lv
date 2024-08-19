use std::{
    collections::{BTreeMap, HashMap, HashSet},
    time::{SystemTime, UNIX_EPOCH},
};
use crate::{verify_access_token, Context, Token};

use actix_web::Result;
use async_graphql::{Json, Object, SimpleObject};
use chrono::Utc;
use entity::{
    advert::{self, Entity as Advert}, favorites::{self, Entity as Favorites}, reviews, specifications::{self, Entity as Specifications}, user::{self, Entity as User}
};
use jwt::VerifyWithKey;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, DeleteResult, EntityTrait,
    ModelTrait, Order, QueryFilter, QueryOrder, QuerySelect, Set,
};


#[derive(SimpleObject)]
struct AdvertWithUser {
    advert: advert::Model,
    user: user::Model,
    is_admin: bool,
    belongs_to_user: bool,
}


#[derive(Default)]
pub struct AdvertQuery;


#[Object]
impl AdvertQuery {
    async fn advert(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
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

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone())  {
            Some(token) => token,
            None => {
                return Ok(AdvertWithUser {
                    advert,
                    user,
                    is_admin: false,
                    belongs_to_user: false,
                });
            }
        };

  
        let claims = match verify_access_token(access_token, &my_ctx.access_key) {
            Ok(claims) => claims,
            Err(err) => return Err(err),
        };


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

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone())  {
            Some(token) => token,
            None => {
                return Ok(adverts);
            }
        };
        
        for advert in &mut adverts {
            let specs: Vec<specifications::Model> =
                advert.find_related(Specifications).all(&my_ctx.db).await?;
            advert.specs = specs;
        }

        let claims = match verify_access_token(access_token, &my_ctx.access_key) {
            Ok(claims) => claims,
            Err(err) => return Err(err),
        };

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

    async fn get_favorites(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Result<Vec<advert::Model>, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone())  {
            Some(token) => token,
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


#[derive(Default)]
pub struct AdvertMutation;

#[Object]
impl AdvertMutation {
    async fn edit_advert(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
        #[graphql(validator(minimum = 0))] price: f32,
        location: String,
        title: String,
        description: String,
        photos: Vec<String>,
    ) -> Result<advert::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone())  {
            Some(token) => token,
            None => {
                return Err(async_graphql::Error::new(
                    "you are not logged in".to_string(),
                ));
            }
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

    async fn add_favorite(
        &self,
        ctx: &async_graphql::Context<'_>,
        advert_id: i32,
    ) -> Result<favorites::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone())  {
            Some(token) => token,
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

        let favorite = favorites::ActiveModel {
            advert_id: Set(advert_id),
            user_id: Set(user.id),
            created_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        let favorite: favorites::Model = favorite.insert(&my_ctx.db).await?;

        return Ok(favorite);
    }

    async fn remove_favorite(
        &self,
        ctx: &async_graphql::Context<'_>,
        advert_id: i32,
    ) -> Result<favorites::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone())  {
            Some(token) => token,
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

    async fn create_advert(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(validator(minimum = 0))] price: f32,
        location: String,
        title: String,
        description: String,
        category: String,
        photos: Vec<String>,
        data: Json<serde_json::Value>,
    ) -> Result<advert::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone())  {
            Some(token) => token,
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

        let naive_date_time = Utc::now().naive_utc();

        let photo_url = photos[0].clone();

        let additional_photos: Vec<String> = photos[1..].iter().cloned().collect();

        let advert = advert::ActiveModel {
            available: Set(true),
            user_id: Set(claims["id"].parse().unwrap()),
            created_at: Set(naive_date_time),
            updated_at: Set(naive_date_time),
            price: Set(price),
            old_price: Set(price),
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
        advert_id: i32,
    ) -> Result<advert::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone())  {
            Some(token) => token,
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

    async fn write_review(
        &self,
        ctx: &async_graphql::Context<'_>,
        advert_id: i32,
        #[graphql(validator(maximum = 5, minimum = 1))] rating: i32,
        message: String,
    ) -> Result<reviews::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone())  {
            Some(token) => token,
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

        let user_id = claims["id"].parse::<i32>().unwrap();

        let adv = Advert::find_by_id(advert_id).one(&my_ctx.db).await?;
        
        let new_advert = match adv {
            Some(a) => a,
            None => return Err(async_graphql::Error::new("advert not found".to_string())),
        };


        if new_advert.user_id == user_id {
            return Err(async_graphql::Error::new("You can't review your own advert".to_string()));
        }

        

        let review = reviews::ActiveModel {
            advert_id: Set(advert_id),
            user_id: Set(user_id),
            rating: Set(rating),
            message: Set(message),
            created_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        let review: reviews::Model = review.insert(&my_ctx.db).await?;


        return Ok(review);
    }

    
}
