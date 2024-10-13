use async_graphql::{self, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Default)]
#[sea_orm(table_name = "user")]
#[graphql(name = "User")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub company_name: Option<String>,
    #[sea_orm(unique)]
    pub email: Option<String>,
    pub phone: Option<String>,
    pub telegram_id: Option<String>,
    pub telegram_username: Option<String>,
    pub balance: f32,
    pub is_admin: bool,
    pub email_verified: bool,
    #[graphql(visible = false)]
    pub password_hash: Option<String>,
    #[sea_orm(ignore)]
    pub adverts: Vec<super::advert::Model>,
    #[sea_orm(ignore)]
    pub adverts_with_reviews: Vec<super::advert::Model>,
    #[sea_orm(ignore)]
    pub reviewed_adverts: Vec<super::advert::Model>,
    #[sea_orm(ignore)]
    pub rating: f32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::advert::Entity")]
    Advert,

    #[sea_orm(has_many = "super::payment::Entity")]
    Payment,

    #[sea_orm(has_many = "super::favorites::Entity")]
    Favorites,

    #[sea_orm(has_many = "super::reviews::Entity", on_delete = "Cascade")]
    Review,
}

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

impl Related<super::reviews::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Review.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_email(email: String) -> Select<Entity> {
        Self::find().filter(Column::Email.eq(email))
    }

    pub fn find_by_telegram_id(telegram_id: String) -> Select<Entity> {
        Self::find().filter(Column::TelegramId.eq(telegram_id))
    }

    pub fn find_by_phone(phone: String) -> Select<Entity> {
        Self::find().filter(Column::Phone.eq(phone))
    }
}
