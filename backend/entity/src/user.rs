use async_graphql::{self, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "user")]
#[graphql(name = "User")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub surname: String,
    #[sea_orm(unique)]
    pub email: String,
    pub phone: String,
    pub balance: f32,
    #[graphql(visible = false)]
    pub password_hash: String,
    #[graphql(visible = false)]
    pub refresh_token: Option<String>,
    // pub adverts
    #[sea_orm(ignore)]
    pub adverts: Vec<super::advert::Model>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::advert::Entity")]
    Advert,
    #[sea_orm(has_many = "super::favorites::Entity")]
    Favorites,
}

// `Related` trait has to be implemented by hand
impl Related<super::advert::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Advert.def()
    }
}

impl Related<super::favorites::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Favorites.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_email(email: String) -> Select<Entity> {
        Self::find().filter(Column::Email.eq(email))
    }
}
