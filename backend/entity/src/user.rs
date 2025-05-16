use async_graphql::{self, Enum, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, EnumIter, DeriveActiveEnum)]
#[sea_orm(enum_name = "role", db_type = "Enum", rs_type = "String")]
#[derive(Default)]
pub enum Role {
    #[sea_orm(string_value = "A")]
    Admin,
    #[sea_orm(string_value = "U")]
    #[default]
    User,
    #[sea_orm(string_value = "M")]
    Moderator,
}

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
    pub email_verified: bool,
    pub banned: bool,
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
    pub role: Role,
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

impl Related<super::chat::Entity> for Entity {
    fn to() -> RelationDef {
        // This defines the relation for chats where the user is the second participant.
        super::chat::Relation::Participant.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_email(email: String) -> Select<Entity> {
        Self::find().filter(Column::Email.eq(email))
    }

    pub fn find_by_phone(phone: String) -> Select<Entity> {
        Self::find().filter(Column::Phone.eq(phone))
    }
}
