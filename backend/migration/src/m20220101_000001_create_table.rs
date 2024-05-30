use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .col(ColumnDef::new(User::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(User::UpdatedAt).date_time().not_null())
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::Surname).string().not_null())
                    .col(ColumnDef::new(User::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(User::Phone).string().not_null())
                    .col(ColumnDef::new(User::Balance).float().not_null())
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    .col(ColumnDef::new(User::RefreshToken).string())
                    .col(
                        ColumnDef::new(User::IsAdmin)
                            .boolean()
                            .not_null()
                            .default("false"),
                    )
                    .to_owned(),
            )
            .await?;
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
                    .col(ColumnDef::new(Advert::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Advert::UpdatedAt).date_time().not_null())
                    .col(ColumnDef::new(Advert::Available).boolean().not_null())
                    .col(ColumnDef::new(Advert::Price).float().not_null())
                    .col(ColumnDef::new(Advert::Location).string().not_null())
                    .col(ColumnDef::new(Advert::Title).string().not_null())
                    .col(ColumnDef::new(Advert::Category).string().not_null())
                    .col(ColumnDef::new(Advert::Description).string().not_null())
                    .col(ColumnDef::new(Advert::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-advert-user_id")
                            .from(Advert::Table, Advert::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;
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
                    .col(ColumnDef::new(Favorites::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Favorites::UserId).integer().not_null())
                    .col(ColumnDef::new(Favorites::AdvertId).integer().not_null())
                    .col(
                        ColumnDef::new(Favorites::String)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
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
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Advert::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Specifications::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Favorites::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    Name,
    Surname,
    Email,
    Phone,
    Balance,
    PasswordHash,
    RefreshToken,
    IsAdmin,
}

#[derive(DeriveIden)]
enum Advert {
    Table,
    Id,
    Title,
    Description,
    CreatedAt,
    UpdatedAt,
    Available,
    Price,
    Location,
    UserId,
    Category,
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
    String,
}
