use async_graphql::{self, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;

/// The `User` entity represents the `user` table in the database.
/// It integrates with GraphQL via `async_graphql` and SeaORM for ORM functionality.
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
    pub email: Option<String>,
    pub phone: Option<String>,
    pub telegram_id: Option<String>,
    pub balance: f32,
    pub is_admin: bool,
    pub email_verified: bool,
    #[graphql(visible = false)]
    pub password_hash: Option<String>,
    #[graphql(visible = false)]
    pub refresh_token: Option<String>,
    #[sea_orm(ignore)]
    pub adverts: Vec<super::advert::Model>,
    #[sea_orm(ignore)]
    pub reviews: Vec<super::reviews::Model>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Defines a one-to-many relationship between `User` and `Advert` via `user_id`.
    #[sea_orm(has_many = "super::advert::Entity")]
    Advert,

    /// Defines a one-to-many relationship between `User` and `Favorites` via `user_id`.
    #[sea_orm(has_many = "super::favorites::Entity")]
    Favorites,

    /// Defines a one-to-many relationship between `User` and `Reviews` via `user_id`.
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
    /// Finds a user by their email address.
    ///
    /// # Arguments
    ///
    /// * `email` - A `String` representing the user's email address.
    ///
    /// # Returns
    ///
    /// A `Select<Entity>` query that can be executed to retrieve the user.
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
