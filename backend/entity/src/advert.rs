use async_graphql::{self, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "advert")]
#[graphql(name = "Advert")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub category: String,
    pub title: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub photo_url: String,
    pub additional_photos: Vec<String>,
    pub available: bool,
    pub price: f32,
    pub location: String,
    pub user_id: i32,
    pub old_price: f32,
    pub sold_to: Option<i32>,

    #[sea_orm(ignore)]
    pub is_favorited: bool,
    #[sea_orm(ignore)]
    pub specs: Vec<super::specifications::Model>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(has_many = "super::specifications::Entity", on_delete = "Cascade")]
    Specifications,
    #[sea_orm(has_many = "super::favorites::Entity", on_delete = "Cascade")]
    Favorites,
    #[sea_orm(has_one = "super::reviews::Entity", on_delete = "Cascade")]
    Review,
    // #[sea_orm(
    //     belongs_to = "super::user::Entity",
    //     from = "Column::SoldTo",
    //     to = "super::user::Column::Id"
    // )]
    // User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::specifications::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Specifications.def()
    }
}

impl Related<super::favorites::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Favorites.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
