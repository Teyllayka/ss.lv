use async_graphql::{self, Enum, SimpleObject};
use sea_orm::entity::prelude::*;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, EnumIter, DeriveActiveEnum)]
#[sea_orm(
    enum_name = "status",       // Optional: Specifies the enum name in the database
    db_type = "Enum",         // Change to "Integer" if you prefer numeric enums
    rs_type = "String",         // Specifies the Rust type for the enum mapping
)]
pub enum Status {
    #[sea_orm(string_value = "P")]
    Pending,
    #[sea_orm(string_value = "C")]

    Completed,
    #[sea_orm(string_value = "F")]

    Failed,
}




#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "payment")]
#[graphql(name = "payment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub order_id: String,
    pub user_id: i32,
    pub amount : f32,
    pub status: Status,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
