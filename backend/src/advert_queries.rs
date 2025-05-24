use crate::{verify_access_token, Context, Token};
use std::collections::{HashMap, HashSet};

use actix_web::Result;
use async_graphql::{Json, Object};
use chrono::Utc;
use entity::{
    advert::{self, Entity as Advert},
    favorites::{self, Entity as Favorites},
    reviews::{self, Entity as Reviews},
    specifications::{self, Entity as Specifications},
    user::{self, Entity as User, Role},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, DeleteResult, EntityTrait, ModelTrait, Order,
    QueryFilter, QueryOrder, QuerySelect, QueryTrait, Set,
};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct AdvertQuery;

#[derive(Deserialize, Serialize)]
struct CustomSpecField {
    key: String,
    value: String,
}

#[Object]
impl AdvertQuery {
    async fn advert(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
    ) -> Result<advert::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().expect("Failed to get context");

        let advert = Advert::find_by_id(id)
            .one(&my_ctx.db)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Advert not found"))?;

        let specs = Specifications::find()
            .filter(specifications::Column::AdvertId.eq(id))
            .all(&my_ctx.db)
            .await?;

        let user = User::find_by_id(advert.user_id)
            .one(&my_ctx.db)
            .await?
            .ok_or_else(|| async_graphql::Error::new("User not found"))?;

        let mut updated_advert = advert.clone();
        updated_advert.specs = specs;

        let mut is_favorited = false;
        let mut user_rating = 0.0;

        if let Some(token) = ctx.data_opt::<Token>() {
            if let Ok(claims) = verify_access_token(token.0.clone(), &my_ctx.access_key) {
                let req_user_id: i32 =
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

                is_favorited = Favorites::find()
                    .filter(favorites::Column::UserId.eq(req_user_id))
                    .filter(favorites::Column::AdvertId.eq(id))
                    .one(&my_ctx.db)
                    .await?
                    .is_some();

                let user_reviews = Reviews::find()
                    .filter(reviews::Column::AdvertId.eq(id))
                    .all(&my_ctx.db)
                    .await?;

                let (total, count) = user_reviews.iter().fold((0.0, 0), |acc, review| {
                    (acc.0 + review.rating as f32, acc.1 + 1)
                });

                if count > 0 {
                    user_rating = total / count as f32;
                }
            }
        }

        updated_advert.is_favorited = is_favorited;
        updated_advert.user = user.clone();
        updated_advert.user.rating = user_rating;

        let review = Reviews::find()
            .filter(reviews::Column::AdvertId.eq(id))
            .one(&my_ctx.db)
            .await?;

        if let Some(mut review) = review {
            let review_user = User::find_by_id(review.user_id)
                .one(&my_ctx.db)
                .await?
                .ok_or_else(|| async_graphql::Error::new("User not found"))?;

            review.user = review_user;

            updated_advert.review = Some(review.clone());
        } else {
            updated_advert.review = None;
        }

        Ok(updated_advert)
    }

    async fn similar_adverts(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
    ) -> Result<Vec<advert::Model>, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let advert = Advert::find_by_id(id)
            .one(&my_ctx.db)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Advert not found"))?;

        let main_advert_specs = advert
            .find_related(specifications::Entity)
            .all(&my_ctx.db)
            .await?;
        let main_specs_set: HashSet<(String, String)> = main_advert_specs
            .iter()
            .map(|spec| (spec.key.clone(), spec.value.clone()))
            .collect();

        let adverts_with_specs = Advert::find()
            .filter(advert::Column::Category.eq(advert.category.clone()))
            .filter(advert::Column::Id.ne(id))
            .filter(advert::Column::Available.eq(true))
            .find_with_related(specifications::Entity)
            .all(&my_ctx.db)
            .await?;

        let mut matching_adverts = Vec::new();
        for (other_advert, specs) in &adverts_with_specs {
            let other_specs_set: HashSet<(String, String)> = specs
                .iter()
                .map(|spec| (spec.key.clone(), spec.value.clone()))
                .collect();
            if other_specs_set == main_specs_set {
                matching_adverts.push(other_advert.clone());
                if matching_adverts.len() >= 4 {
                    break;
                }
            }
        }

        if matching_adverts.len() < 4 {
            let mut adverts_by_matching_specs: Vec<(advert::Model, usize)> = adverts_with_specs
                .iter()
                .map(|(other_advert, specs)| {
                    let other_specs_set: HashSet<(String, String)> = specs
                        .iter()
                        .map(|spec| (spec.key.clone(), spec.value.clone()))
                        .collect();
                    let matching_specs_count =
                        main_specs_set.intersection(&other_specs_set).count();
                    (other_advert.clone(), matching_specs_count)
                })
                .collect();
            adverts_by_matching_specs.sort_by(|a, b| b.1.cmp(&a.1));
            for (advert, _) in adverts_by_matching_specs {
                if !matching_adverts.contains(&advert) {
                    matching_adverts.push(advert);
                    if matching_adverts.len() >= 4 {
                        break;
                    }
                }
            }
        }

        if matching_adverts.is_empty() {
            matching_adverts = Advert::find()
                .filter(advert::Column::Category.eq(advert.category.clone()))
                .filter(advert::Column::Id.ne(id))
                .filter(advert::Column::Available.eq(true))
                .limit(4)
                .all(&my_ctx.db)
                .await?;
        }

        let advert_ids: Vec<i32> = matching_adverts.iter().map(|adv| adv.id).collect();
        let specs = Specifications::find()
            .filter(specifications::Column::AdvertId.is_in(advert_ids.clone()))
            .all(&my_ctx.db)
            .await?;
        let mut specs_map: HashMap<i32, Vec<specifications::Model>> = HashMap::new();
        for spec in specs {
            specs_map.entry(spec.advert_id).or_default().push(spec);
        }

        let user_ids: HashSet<i32> = matching_adverts.iter().map(|adv| adv.user_id).collect();
        let users = User::find()
            .filter(user::Column::Id.is_in(user_ids.clone()))
            .all(&my_ctx.db)
            .await?;
        let users_map: HashMap<i32, user::Model> =
            users.into_iter().map(|user| (user.id, user)).collect();

        let user_adverts = Advert::find()
            .filter(advert::Column::UserId.is_in(user_ids.clone()))
            .all(&my_ctx.db)
            .await?;
        let user_advert_ids: Vec<i32> = user_adverts.iter().map(|adv| adv.id).collect();

        let reviews = Reviews::find()
            .filter(reviews::Column::AdvertId.is_in(user_advert_ids.clone()))
            .all(&my_ctx.db)
            .await?;
        let mut user_ratings: HashMap<i32, (f32, usize)> = HashMap::new();
        let advert_user_map: HashMap<i32, i32> = user_adverts
            .iter()
            .map(|adv| (adv.id, adv.user_id))
            .collect();
        for review in reviews {
            if let Some(&user_id) = advert_user_map.get(&review.advert_id) {
                let entry = user_ratings.entry(user_id).or_insert((0.0, 0));
                entry.0 += review.rating as f32;
                entry.1 += 1;
            }
        }

        let mut favorite_advert_ids = HashSet::new();
        if let Some(token) = ctx.data_opt::<Token>() {
            if let Ok(claims) = verify_access_token(token.0.clone(), &my_ctx.access_key) {
                let user_id: i32 = if let Some(id_str) = claims.get("id").and_then(|v| v.as_str()) {
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
                let favorite_adverts = Favorites::find()
                    .filter(favorites::Column::UserId.eq(user_id))
                    .all(&my_ctx.db)
                    .await?;
                favorite_advert_ids = favorite_adverts.iter().map(|fav| fav.advert_id).collect();
            }
        }

        let enriched_adverts: Vec<advert::Model> = matching_adverts
            .into_iter()
            .map(|mut advert| {
                advert.specs = specs_map.get(&advert.id).cloned().unwrap_or_default();
                if let Some(user) = users_map.get(&advert.user_id) {
                    advert.user = user.clone();
                    let user_rating = user_ratings
                        .get(&user.id)
                        .map(|(total, count)| total / *count as f32)
                        .unwrap_or(0.0);
                    advert.user.rating = user_rating;
                }
                advert.is_favorited = favorite_advert_ids.contains(&advert.id);
                advert
            })
            .collect();

        Ok(enriched_adverts)
    }

    async fn get_adverts(
        &self,
        ctx: &async_graphql::Context<'_>,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<advert::Model>, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();
        let adverts: Vec<advert::Model> = advert::Entity::find()
            .order_by(advert::Column::Id, Order::Desc)
            .offset(offset as u64)
            .limit(limit as u64)
            .filter(advert::Column::Available.eq(true))
            .all(&my_ctx.db)
            .await?;

        let advert_ids: Vec<i32> = adverts.iter().map(|adv| adv.id).collect();

        let specs = Specifications::find()
            .filter(specifications::Column::AdvertId.is_in(advert_ids))
            .all(&my_ctx.db)
            .await?;

        let mut specs_map: HashMap<i32, Vec<specifications::Model>> = HashMap::new();
        for spec in specs {
            specs_map.entry(spec.advert_id).or_default().push(spec);
        }

        let user_ids: HashSet<i32> = adverts.iter().map(|adv| adv.user_id).collect();
        let users = User::find()
            .filter(user::Column::Id.is_in(user_ids.clone()))
            .all(&my_ctx.db)
            .await?;

        let users_map: HashMap<i32, user::Model> =
            users.into_iter().map(|user| (user.id, user)).collect();

        let user_adverts = Advert::find()
            .filter(advert::Column::UserId.is_in(user_ids.clone()))
            .all(&my_ctx.db)
            .await?;

        let user_advert_ids: Vec<i32> = user_adverts.iter().map(|adv| adv.id).collect();

        let reviews = Reviews::find()
            .filter(reviews::Column::AdvertId.is_in(user_advert_ids.clone()))
            .all(&my_ctx.db)
            .await?;

        let mut user_ratings: HashMap<i32, (f32, usize)> = HashMap::new();
        let advert_user_map: HashMap<i32, i32> = user_adverts
            .iter()
            .map(|adv| (adv.id, adv.user_id))
            .collect();

        for review in reviews {
            if let Some(&user_id) = advert_user_map.get(&review.advert_id) {
                let entry = user_ratings.entry(user_id).or_insert((0.0, 0));
                entry.0 += review.rating as f32;
                entry.1 += 1;
            }
        }

        let mut favorite_advert_ids = HashSet::new();
        if let Some(token) = ctx.data_opt::<Token>() {
            let claims = verify_access_token(token.0.clone(), &my_ctx.access_key);

            if let Some(claims) = claims.ok() {
                let user_id: i32 = if let Some(id_str) = claims.get("id").and_then(|v| v.as_str()) {
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

                let favorite_adverts = Favorites::find()
                    .filter(favorites::Column::UserId.eq(user_id))
                    .all(&my_ctx.db)
                    .await?;

                favorite_advert_ids = favorite_adverts.iter().map(|fav| fav.advert_id).collect();
            }
        }

        let result: Vec<advert::Model> = adverts
            .into_iter()
            .map(|mut advert| {
                let specs = specs_map.get(&advert.id).cloned().unwrap_or_default();
                let user = users_map.get(&advert.user_id).cloned();
                let user_rating = user
                    .as_ref()
                    .and_then(|u| user_ratings.get(&u.id))
                    .map(|(total, count)| total / *count as f32);
                let is_favorited = favorite_advert_ids.contains(&advert.id);

                advert.is_favorited = is_favorited;
                advert.specs = specs;
                advert.user = user.unwrap();
                advert.user.rating = user_rating.unwrap_or(0.0);

                advert
            })
            .collect();

        return Ok(result);
    }

    pub async fn search_adverts(
        &self,
        ctx: &async_graphql::Context<'_>,
        category: Option<String>,
        offset: i32,
        title: String,
        min_price: Option<f32>,
        max_price: Option<f32>,
        min_rating: Option<f32>,
        sort_field: Option<String>,
        sort_order: Option<String>,
        center_lat: Option<f32>,
        center_lon: Option<f32>,
        location_range: Option<f32>,
        custom_fields: Option<async_graphql::Json<serde_json::Value>>,
    ) -> Result<Vec<advert::Model>, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let mut query = advert::Entity::find()
            .filter(advert::Column::Available.eq(true))
            .filter(advert::Column::Title.contains(&title));

        if let Some(cat) = category {
            query = query.filter(advert::Column::Category.eq(cat));
        }

        if let Some(min) = min_price {
            query = query.filter(advert::Column::Price.gte(min));
        }
        if let Some(max) = max_price {
            query = query.filter(advert::Column::Price.lte(max));
        }

        if let Some(fields) = custom_fields {
            if let Some(fields) = fields.as_object() {
                for (key, value) in fields {
                    let subquery = specifications::Entity::find()
                        .filter(specifications::Column::Key.eq(key.clone()))
                        .filter(
                            specifications::Column::Value
                                .eq(value.as_str().unwrap_or_default().to_string()),
                        )
                        .select_only()
                        .column(specifications::Column::AdvertId)
                        .into_query();

                    query = query.filter(advert::Column::Id.in_subquery(subquery));
                }
            }
        }

        if let (Some(lat), Some(lon), Some(range)) = (center_lat, center_lon, location_range) {
            query = query
                .filter(advert::Column::Lat.between(lat - (range / 111.0), lat + (range / 111.0)))
                .filter(advert::Column::Lon.between(
                    lon - (range / (111.0 * (lat.cos()))),
                    lon + (range / (111.0 * (lat.cos()))),
                ));
        }

        let mut adverts = query.all(&my_ctx.db).await?;

        let advert_ids: Vec<i32> = adverts.iter().map(|adv| adv.id).collect();
        let reviews = Reviews::find()
            .filter(reviews::Column::AdvertId.is_in(advert_ids.clone()))
            .all(&my_ctx.db)
            .await?;

        let mut ratings_map: std::collections::HashMap<i32, (f32, usize)> =
            std::collections::HashMap::new();
        for review in reviews {
            let entry = ratings_map.entry(review.advert_id).or_insert((0.0, 0));
            entry.0 += review.rating as f32;
            entry.1 += 1;
        }

        for advert in &mut adverts {
            let avg_rating = if let Some(&(total, count)) = ratings_map.get(&advert.id) {
                if count > 0 {
                    total / count as f32
                } else {
                    0.0
                }
            } else {
                0.0
            };
            advert.user.rating = avg_rating;
        }

        if let Some(min_rating) = min_rating {
            adverts = adverts
                .into_iter()
                .filter(|adv| adv.user.rating >= min_rating)
                .collect();
        }

        if let Some(field) = sort_field {
            let order = sort_order
                .map(|s| s.to_lowercase())
                .unwrap_or_else(|| "desc".to_string());
            match field.as_str() {
                "rating" => {
                    if order == "asc" {
                        adverts.sort_by(|a, b| {
                            a.user
                                .rating
                                .partial_cmp(&b.user.rating)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        });
                    } else {
                        adverts.sort_by(|a, b| {
                            b.user
                                .rating
                                .partial_cmp(&a.user.rating)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        });
                    }
                }
                "price" => {
                    if order == "asc" {
                        adverts.sort_by(|a, b| {
                            a.price
                                .partial_cmp(&b.price)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        });
                    } else {
                        adverts.sort_by(|a, b| {
                            b.price
                                .partial_cmp(&a.price)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        });
                    }
                }
                "title" => {
                    if order == "asc" {
                        adverts.sort_by(|a, b| a.title.cmp(&b.title));
                    } else {
                        adverts.sort_by(|a, b| b.title.cmp(&a.title));
                    }
                }
                _ => {} 
            }
        }

        let offset = offset as usize;
        let adverts = if offset < adverts.len() {
            adverts.into_iter().skip(offset).collect()
        } else {
            vec![]
        };

        Ok(adverts)
    }

    async fn get_favorites(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Result<Vec<advert::Model>, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone()) {
            Some(token) => token,
            None => {
                return Err(async_graphql::Error::new("You are not logged in."));
            }
        };

        let claims = match verify_access_token(access_token, &my_ctx.access_key) {
            Ok(claims) => claims,
            Err(err) => {
                return Err(async_graphql::Error::new(format!(
                    "Invalid token: {:?}",
                    err
                )))
            }
        };

        let user_id: i32 = if let Some(id_str) = claims.get("id").and_then(|v| v.as_str()) {
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

        let user: Option<user::Model> = User::find_by_id(user_id).one(&my_ctx.db).await?;
        match user {
            Some(user) => user,
            None => return Err(async_graphql::Error::new("User not found.")),
        };

        let favorites: Vec<favorites::Model> = Favorites::find()
            .filter(favorites::Column::UserId.eq(user_id))
            .all(&my_ctx.db)
            .await?;

        if favorites.is_empty() {
            return Ok(vec![]);
        }

        let favorite_advert_ids: Vec<i32> = favorites.iter().map(|fav| fav.advert_id).collect();

        let adverts: Vec<advert::Model> = Advert::find()
            .filter(advert::Column::Id.is_in(favorite_advert_ids.clone()))
            .all(&my_ctx.db)
            .await?;

        if adverts.is_empty() {
            return Err(async_graphql::Error::new(
                "No adverts found for the given favorites.",
            ));
        }

        let specs = Specifications::find()
            .filter(specifications::Column::AdvertId.is_in(favorite_advert_ids.clone()))
            .all(&my_ctx.db)
            .await?;

        let mut specs_map: HashMap<i32, Vec<specifications::Model>> = HashMap::new();
        for spec in specs {
            specs_map.entry(spec.advert_id).or_default().push(spec);
        }

        let user_ids: HashSet<i32> = adverts.iter().map(|adv| adv.user_id).collect();
        let users = User::find()
            .filter(user::Column::Id.is_in(user_ids.clone()))
            .all(&my_ctx.db)
            .await?;

        let users_map: HashMap<i32, user::Model> = users.into_iter().map(|u| (u.id, u)).collect();

        let user_adverts = Advert::find()
            .filter(advert::Column::UserId.is_in(user_ids.clone()))
            .all(&my_ctx.db)
            .await?;

        let user_advert_ids: Vec<i32> = user_adverts.iter().map(|adv| adv.id).collect();

        let reviews = Reviews::find()
            .filter(reviews::Column::AdvertId.is_in(user_advert_ids.clone()))
            .all(&my_ctx.db)
            .await?;

        let mut user_ratings: HashMap<i32, (f32, usize)> = HashMap::new();
        let advert_user_map: HashMap<i32, i32> = user_adverts
            .iter()
            .map(|adv| (adv.id, adv.user_id))
            .collect();

        for review in reviews {
            if let Some(&u_id) = advert_user_map.get(&review.advert_id) {
                let entry = user_ratings.entry(u_id).or_insert((0.0, 0));
                entry.0 += review.rating as f32;
                entry.1 += 1;
            }
        }

        let result: Vec<advert::Model> = adverts
            .into_iter()
            .map(|mut advert| {
                advert.specs = specs_map.get(&advert.id).cloned().unwrap_or_default();

                if let Some(user) = users_map.get(&advert.user_id) {
                    advert.user = user.clone();

                    if let Some((total, count)) = user_ratings.get(&user.id) {
                        advert.user.rating = total / (*count as f32);
                    } else {
                        advert.user.rating = 0.0;
                    }
                }

                advert.is_favorited = true;

                advert
            })
            .collect();

        Ok(result)
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
        lat: f32,
        lon: f32,
        title: String,
        description: String,
        #[graphql(validator(list, min_items = 1))] photos: Vec<String>,
    ) -> Result<advert::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone()) {
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

        let user_id: i32 = if let Some(id_str) = claims.get("id").and_then(|v| v.as_str()) {
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

        let advert: Option<advert::Model> = Advert::find_by_id(id).one(&my_ctx.db).await?;
        println!("advert: {:?}", advert);

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
            additional_photos = advert.additional_photos.clone().unwrap();
        }

        println!("photo_url: {}, {:?}", photo_url, additional_photos);

        let new_advert = advert::ActiveModel {
            photo_url: Set(photo_url),
            additional_photos: Set(Some(additional_photos)),
            price: Set(price),
            lat: Set(lat),
            lon: Set(lon),
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

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone()) {
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

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone()) {
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
        lat: f32,
        lon: f32,
        title: String,
        description: String,
        category: String,
        #[graphql(validator(list, min_items = 1))] photos: Vec<String>,
        data: Json<serde_json::Value>,
    ) -> Result<advert::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone()) {
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

        let id = if let Some(id_str) = claims.get("id").and_then(|v| v.as_str()) {
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

        let user_q: Option<user::Model> = User::find_by_id(id).one(&my_ctx.db).await?;

        match user_q {
            Some(user) => {
                if !user.email_verified {
                    return Err(async_graphql::Error::new(
                        "You are not verified".to_string(),
                    ));
                }
                if user.banned {
                    return Err(async_graphql::Error::new("You are banned".to_string()));
                }
            }
            None => return Err(async_graphql::Error::new("Wrong token".to_string())),
        }

        let naive_date_time = Utc::now().naive_utc();

        let photo_url = photos[0].clone();

        let additional_photos: Vec<String> = photos[1..].iter().cloned().collect();

        let advert = advert::ActiveModel {
            available: Set(true),
            user_id: Set(
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
                },
            ),
            created_at: Set(naive_date_time),
            updated_at: Set(naive_date_time),
            price: Set(price),
            old_price: Set(price),
            lat: Set(lat),
            lon: Set(lon),
            description: Set(description),
            title: Set(title),
            category: Set(category),
            photo_url: Set(photo_url),
            additional_photos: Set(Some(additional_photos)),
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

        let advert: Option<advert::Model> = Advert::find_by_id(advert_id).one(&my_ctx.db).await?;

        let advert = match advert {
            Some(advert) => advert,
            None => return Err(async_graphql::Error::new("Advert not found.")),
        };

        let user_id = if let Some(id_str) = claims.get("id").and_then(|v| v.as_str()) {
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

        let req_user: Option<user::Model> = User::find_by_id(user_id).one(&my_ctx.db).await?;

        match req_user {
            Some(req_user) => {
                if req_user.role == Role::Admin || req_user.role == Role::Moderator {
                } else if advert.user_id == user_id {
                    if !advert.sold_to.is_none() {
                        return Err(async_graphql::Error::new(
                            "You cannot delete this advert as it has already been sold.",
                        ));
                    }
                } else {
                    return Err(async_graphql::Error::new(
                        "You do not have the rights to delete this advert.",
                    ));
                }
            }
            None => return Err(async_graphql::Error::new("Invalid token.")),
        };

        advert.clone().delete(&my_ctx.db).await?;

        Ok(advert)
    }

    async fn write_review(
        &self,
        ctx: &async_graphql::Context<'_>,
        advert_id: i32,
        #[graphql(validator(maximum = 5, minimum = 1))] rating: i32,
        message: String,
    ) -> Result<reviews::Model, async_graphql::Error> {
        let my_ctx = ctx.data::<Context>().unwrap();

        let access_token = match ctx.data_opt::<Token>().map(|token| token.0.clone()) {
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

        let user_id = if let Some(id_str) = claims.get("id").and_then(|v| v.as_str()) {
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

        let adv = Advert::find_by_id(advert_id).one(&my_ctx.db).await?;

        let new_advert = match adv {
            Some(a) => a,
            None => return Err(async_graphql::Error::new("advert not found".to_string())),
        };

        if new_advert.user_id == user_id {
            return Err(async_graphql::Error::new(
                "You can't review your own advert".to_string(),
            ));
        }

        if new_advert.sold_to != Some(user_id) {
            return Err(async_graphql::Error::new(
                "You can't review this advert as it has not been sold to you".to_string(),
            ));
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
