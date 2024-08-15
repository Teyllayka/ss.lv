use async_graphql::{self, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "user")]
#[graphql(name = "User")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub avatar_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub surname: String,
    #[sea_orm(unique)]
    pub email: String,
    pub phone: String,
    pub balance: f32,
    pub is_admin: bool,
    #[graphql(visible = false)]
    pub password_hash: String,
    #[graphql(visible = false)]
    pub refresh_token: Option<String>,
    // pub adverts
    #[sea_orm(ignore)]
    pub adverts: Vec<super::advert::Model>,
    #[sea_orm(ignore)]
    pub reviews: Vec<super::reviews::Model>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::advert::Entity")]
    Advert,
    #[sea_orm(has_many = "super::favorites::Entity")]
    Favorites,
    #[sea_orm(has_many = "super::reviews::Entity", on_delete = "Cascade")]
    Review,
    // #[sea_orm(has_many = "super::advert::Entity")]
    // Advert,
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
