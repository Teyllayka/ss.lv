use async_graphql::{self, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, Default)]
#[sea_orm(table_name = "chat")]
#[graphql(name = "Chat")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub advert_id: i32,
    pub participant_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
        from = "Column::ParticipantId",
        to = "super::user::Column::Id"
    )]
    Participant,
    #[sea_orm(has_many = "super::message::Entity", on_delete = "Cascade")]
    Messages,
    #[sea_orm(has_one = "super::deal::Entity", on_delete = "Cascade")]
    Deal,
}

impl Related<super::advert::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Advert.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Participant.def()
    }
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Messages.def()
    }
}

impl Related<super::deal::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Deal.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
