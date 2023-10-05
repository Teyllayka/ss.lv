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
                    .table(Specification::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Specification::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Specification::Key).string().not_null())
                    .col(ColumnDef::new(Specification::Value).string().not_null())
                    .col(ColumnDef::new(Specification::AdvertId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-specification-advert_id")
                            .from(Specification::Table, Specification::AdvertId)
                            .to(Advert::Table, Advert::Id),
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
            .drop_table(Table::drop().table(Specification::Table).to_owned())
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
}

#[derive(DeriveIden)]
enum Specification {
    Table,
    Id,
    Key,
    Value,
    AdvertId,
}
