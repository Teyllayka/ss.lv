use async_graphql::{self, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Default)]
#[sea_orm(table_name = "message")]
#[graphql(name = "Message")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub chat_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::chat::Entity",
        from = "Column::ChatId",
        to = "super::chat::Column::Id"
    )]
    Chat,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::chat::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Chat.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
