use sea_orm_migration::prelude::*;
// use argon2::{
//     password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
//     Argon2,
// };

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
                    .col(ColumnDef::new(User::AvatarUrl).string().not_null())
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
                    .col(ColumnDef::new(Advert::PhotoUrl).string().not_null())
                    .col(ColumnDef::new(Advert::Location).string().not_null())
                    .col(
                        ColumnDef::new(Advert::AdditionalPhotos)
                            .array(ColumnType::String(None))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Advert::Title).string().not_null())
                    .col(ColumnDef::new(Advert::Category).string().not_null())
                    .col(ColumnDef::new(Advert::Description).string().not_null())
                    .col(ColumnDef::new(Advert::UserId).integer().not_null())
                    .col(ColumnDef::new(Advert::SoldTo).integer())
                    .col(ColumnDef::new(Advert::OldPrice).float().not_null())
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
                    .col(ColumnDef::new(Reviews::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Reviews::UserId).integer().not_null())
                    .col(ColumnDef::new(Reviews::AdvertId).integer().not_null())
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

        // let salt = SaltString::generate(&mut OsRng);
        // let argon2 = Argon2::default();

        // let password_hash = argon2.hash_password("test".as_bytes(), &salt).await?;

        // let insert = Query::insert()
        //     .into_table(User::Table)
        //     .columns([User::Name, User::AvatarUrl, User::Surname, User::Email, User::Phone, User::Balance, User::PasswordHash, User::IsAdmin])
        //     .values_panic(["Test".into(), "".into(), "Test".into(), "Test@Test.com".into(), "123456789".into(), 0.0.into(), password_hash.into(), true.into()])
        //     .to_owned();

        // manager.exec_stmt(insert).await?;

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
    AvatarUrl,
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
    PhotoUrl,
    AdditionalPhotos,
    Available,
    Price,
    Location,
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
