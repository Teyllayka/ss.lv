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
    pub available: bool,
    pub price: f32,
    pub location: String,
    pub user_id: i32,

    #[sea_orm(ignore)]
    pub is_favorited: bool,
    // pub adverts
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
