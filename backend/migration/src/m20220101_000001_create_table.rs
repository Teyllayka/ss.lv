use sea_orm::{ActiveEnum, DbBackend, DeriveActiveEnum, EnumIter, Schema};
use sea_orm_migration::prelude::*;
// use argon2::{ /* ... */ };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(DbBackend::Postgres);

        // Create ENUM types for Status and Role.
        manager
            .create_type(schema.create_enum_from_active_enum::<Status>())
            .await?;
        manager
            .create_type(schema.create_enum_from_active_enum::<Role>())
            .await?;

        // Create User Table.
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .col(ColumnDef::new(User::AvatarUrl).string().null())
                    .col(ColumnDef::new(User::Name).string().null())
                    .col(ColumnDef::new(User::Surname).string().null())
                    .col(ColumnDef::new(User::CompanyName).string().null())
                    .col(ColumnDef::new(User::Email).string().unique_key().null())
                    .col(ColumnDef::new(User::Phone).string().null())
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    .col(
                        ColumnDef::new(User::EmailVerified)
                            .boolean()
                            .not_null()
                            .default("false"),
                    )
                    .col(ColumnDef::new(User::Banned).boolean().not_null().default("false"))
                    .col(
                        ColumnDef::new(User::Role)
                            .custom(Role::name())
                            .not_null()
                            .default(Expr::value("U")),
                    )
                    .check(
                        Expr::cust("(email IS NOT NULL OR phone IS NOT NULL)").to_owned(),
                    )
                    .check(
                        Expr::cust("((name IS NOT NULL AND surname IS NOT NULL) OR company_name IS NOT NULL)")
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Advert Table.
        manager
            .create_table(
                Table::create()
                    .table(Advert::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Advert::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Advert::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .col(
                        ColumnDef::new(Advert::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .col(ColumnDef::new(Advert::Available).boolean().not_null())
                    .col(ColumnDef::new(Advert::Price).float().not_null())
                    .col(ColumnDef::new(Advert::PhotoUrl).string().not_null())
                    .col(ColumnDef::new(Advert::Lat).float().not_null())
                    .col(ColumnDef::new(Advert::Lon).float().not_null())
                    .col(
                        ColumnDef::new(Advert::AdditionalPhotos)
                            .array(ColumnType::String(StringLen::None))
                            .null(),
                    )
                    .col(ColumnDef::new(Advert::Title).string().not_null())
                    .col(ColumnDef::new(Advert::Category).string().not_null())
                    .col(ColumnDef::new(Advert::Description).string().not_null())
                    .col(ColumnDef::new(Advert::UserId).integer().not_null())
                    .col(ColumnDef::new(Advert::SoldTo).integer().null())
                    .col(ColumnDef::new(Advert::OldPrice).float().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-advert-user_id")
                            .from(Advert::Table, Advert::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-advert-sold_to")
                            .from(Advert::Table, Advert::SoldTo)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Specifications Table.
        manager
            .create_table(
                Table::create()
                    .table(Specifications::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Specifications::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Specifications::Key).string().not_null())
                    .col(ColumnDef::new(Specifications::Value).string().not_null())
                    .col(
                        ColumnDef::new(Specifications::AdvertId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-specification-advert_id")
                            .from(Specifications::Table, Specifications::AdvertId)
                            .to(Advert::Table, Advert::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Favorites Table.
        manager
            .create_table(
                Table::create()
                    .table(Favorites::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Favorites::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Favorites::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .col(ColumnDef::new(Favorites::UserId).integer().not_null())
                    .col(ColumnDef::new(Favorites::AdvertId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-favorites-advert_id")
                            .from(Favorites::Table, Favorites::AdvertId)
                            .to(Advert::Table, Advert::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-favorites-user_id")
                            .from(Favorites::Table, Favorites::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Reviews Table.
        manager
            .create_table(
                Table::create()
                    .table(Reviews::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reviews::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Reviews::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .col(ColumnDef::new(Reviews::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(Reviews::AdvertId)
                            .integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Reviews::Rating).integer().not_null())
                    .col(ColumnDef::new(Reviews::Message).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-reviews-advert_id")
                            .from(Reviews::Table, Reviews::AdvertId)
                            .to(Advert::Table, Advert::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-reviews-user_id")
                            .from(Reviews::Table, Reviews::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Chat::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Chat::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Chat::AdvertId).integer().not_null())
                    .col(ColumnDef::new(Chat::ParticipantId).integer().not_null())
                    .col(
                        ColumnDef::new(Chat::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .col(
                        ColumnDef::new(Chat::Archived)
                            .boolean()
                            .not_null()
                            .default("false"),
                    )
                    .col(
                        ColumnDef::new(Chat::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-chat-advert_id")
                            .from(Chat::Table, Chat::AdvertId)
                            .to(Advert::Table, Advert::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-chat-participant_id")
                            .from(Chat::Table, Chat::ParticipantId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Message Table.
        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Message::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Message::ChatId).integer().not_null())
                    .col(ColumnDef::new(Message::UserId).integer().not_null())
                    .col(ColumnDef::new(Message::Content).string().not_null())
                    .col(
                        ColumnDef::new(Message::Urls)
                            .array(ColumnType::String(StringLen::None))
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Message::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .col(ColumnDef::new(Message::ReadAt).date_time().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-message-chat_id")
                            .from(Message::Table, Message::ChatId)
                            .to(Chat::Table, Chat::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-message-user_id")
                            .from(Message::Table, Message::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Deal::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Deal::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Deal::ChatId)
                            .integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Deal::Price).float().not_null())
                    .col(
                        ColumnDef::new(Deal::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .col(ColumnDef::new(Deal::RequesterId).integer().not_null())
                    .col(ColumnDef::new(Deal::Status).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-deal-chat_id")
                            .from(Deal::Table, Deal::ChatId)
                            .to(Chat::Table, Chat::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse dependency order.
        manager
            .drop_table(Table::drop().table(Deal::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Chat::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Reviews::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Favorites::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Specifications::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Advert::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        Ok(())
    }
}

// --- Table Identifiers --- //

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    AvatarUrl,
    Name,
    Surname,
    CompanyName,
    Email,
    Phone,
    PasswordHash,
    EmailVerified,
    Banned,
    Role,
}

#[derive(DeriveIden)]
enum Advert {
    Table,
    Id,
    Title,
    Description,
    CreatedAt,
    UpdatedAt,
    PhotoUrl,
    AdditionalPhotos,
    Available,
    Price,
    Lat,
    Lon,
    UserId,
    Category,
    SoldTo,
    OldPrice,
}

#[derive(DeriveIden)]
enum Specifications {
    Table,
    Id,
    Key,
    Value,
    AdvertId,
}

#[derive(DeriveIden)]
enum Favorites {
    Table,
    Id,
    AdvertId,
    UserId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Reviews {
    Table,
    Id,
    AdvertId,
    UserId,
    CreatedAt,
    Rating,
    Message,
}

#[derive(DeriveIden)]
enum Chat {
    Table,
    Id,
    AdvertId,
    ParticipantId,
    CreatedAt,
    UpdatedAt,
    Archived,
}

#[derive(DeriveIden)]
enum Message {
    Table,
    Id,
    ChatId,
    UserId,
    Content,
    Urls,
    CreatedAt,
    ReadAt,
}

#[derive(DeriveIden)]
enum Deal {
    Table,
    Id,
    ChatId,
    Price,
    CreatedAt,
    RequesterId,
    Status,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "status")]
enum Status {
    #[sea_orm(string_value = "P")]
    Pending,
    #[sea_orm(string_value = "C")]
    Completed,
    #[sea_orm(string_value = "F")]
    Failed,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "role")]
enum Role {
    #[sea_orm(string_value = "A")]
    Admin,
    #[sea_orm(string_value = "U")]
    User,
    #[sea_orm(string_value = "M")]
    Moderator,
}
