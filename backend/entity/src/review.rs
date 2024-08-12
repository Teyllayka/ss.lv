use async_graphql::{self, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "reviews")]
#[graphql(name = "reviews")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub user_id: i32,
    pub advert_id: i32,
    pub string: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::advert::Entity",
        from = "Column::AdvertId",
        to = "super::advert::Column::Id"
    )]
    Advert,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::advert::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Advert.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
