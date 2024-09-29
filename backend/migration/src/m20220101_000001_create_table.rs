use sea_orm::{ActiveEnum, DbBackend, DeriveActiveEnum, EnumIter, Schema};
use sea_orm_migration::prelude::*;
// use argon2::{ /* ... */ };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create User Table with Updated Constraints
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    // Primary Key
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // Timestamps
                    .col(ColumnDef::new(User::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(User::UpdatedAt).date_time().not_null())
                    // User Details
                    .col(ColumnDef::new(User::AvatarUrl).string().null())
                    .col(ColumnDef::new(User::Name).string().null()) // Made nullable
                    .col(ColumnDef::new(User::Surname).string().null()) // Made nullable
                    .col(ColumnDef::new(User::CompanyName).string().null()) // Added company_name
                    // Contact Information (Nullable)
                    .col(ColumnDef::new(User::Email).string().unique_key().null())
                    .col(ColumnDef::new(User::Phone).string().null())
                    // Telegram ID (Nullable and Unique)
                    .col(ColumnDef::new(User::TelegramId).string().unique_key().null())
                    // Financial Details
                    .col(ColumnDef::new(User::Balance).float().not_null())
                    // Authentication Details (PasswordHash Nullable)
                    .col(ColumnDef::new(User::PasswordHash).string().null())
                    // Administrative Flags
                    .col(
                        ColumnDef::new(User::IsAdmin)
                            .boolean()
                            .not_null()
                            .default("false"),
                    )
                    .col(
                        ColumnDef::new(User::EmailVerified)
                            .boolean()
                            .not_null()
                            .default("false"),
                    )
                    // CHECK Constraints
                    .check(
                        Expr::cust("(email IS NOT NULL OR phone IS NOT NULL)")
                            .to_owned(),
                    )
                    .check(
                        Expr::cust("(password_hash IS NOT NULL OR telegram_id IS NOT NULL)")
                            .to_owned(),
                    )
                    .check(
                        Expr::cust("((name IS NOT NULL AND surname IS NOT NULL) OR company_name IS NOT NULL)")
                            .to_owned(),
                    ) // Added new CHECK constraint
                    .to_owned(),
            )
            .await?;
        
        // Create Advert Table
        manager
            .create_table(
                Table::create()
                    .table(Advert::Table)
                    .if_not_exists()
                    // Primary Key
                    .col(
                        ColumnDef::new(Advert::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // Timestamps
                    .col(ColumnDef::new(Advert::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Advert::UpdatedAt).date_time().not_null())
                    // Advert Details
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
                    // Foreign Keys
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
        
        // Create Specifications Table
        manager
            .create_table(
                Table::create()
                    .table(Specifications::Table)
                    .if_not_exists()
                    // Primary Key
                    .col(
                        ColumnDef::new(Specifications::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // Specification Details
                    .col(ColumnDef::new(Specifications::Key).string().not_null())
                    .col(ColumnDef::new(Specifications::Value).string().not_null())
                    .col(ColumnDef::new(Specifications::AdvertId).integer().not_null())
                    // Foreign Key with Cascade Delete
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
        
        // Create Favorites Table
        manager
            .create_table(
                Table::create()
                    .table(Favorites::Table)
                    .if_not_exists()
                    // Primary Key
                    .col(
                        ColumnDef::new(Favorites::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // Timestamps and Foreign Keys
                    .col(ColumnDef::new(Favorites::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Favorites::UserId).integer().not_null())
                    .col(ColumnDef::new(Favorites::AdvertId).integer().not_null())
                    // Foreign Keys with Cascade Delete
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

            let schema = Schema::new(DbBackend::Postgres);


            manager.create_type(
                schema.create_enum_from_active_enum::<Status>(),
            ).await?;


            manager
            .create_table(
                Table::create()
                    .table(Payment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Payment::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Payment::OrderId).string().not_null())
                    .col(ColumnDef::new(Payment::UserId).integer().not_null())
                    .col(ColumnDef::new(Payment::Amount).float().not_null())
                    .col(
                        ColumnDef::new(Payment::Status).custom(Status::name()).not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-payment-user_id")
                            .from(Payment::Table, Payment::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        
        
        
        // Create Reviews Table
        manager
            .create_table(
                Table::create()
                    .table(Reviews::Table)
                    .if_not_exists()
                    // Primary Key
                    .col(
                        ColumnDef::new(Reviews::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // Review Details
                    .col(ColumnDef::new(Reviews::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Reviews::UserId).integer().not_null())
                    .col(ColumnDef::new(Reviews::AdvertId).integer().not_null().unique_key())
                    .col(ColumnDef::new(Reviews::Rating).integer().not_null())
                    .col(ColumnDef::new(Reviews::Message).string().not_null())
                    // Foreign Keys with Cascade Delete
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
        
        // Optionally, Insert a Default Admin User
        // Uncomment and modify as needed
        /*
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
    
        let password_hash = argon2.hash_password("test".as_bytes(), &salt).await?;
    
        let insert = Query::insert()
            .into_table(User::Table)
            .columns([
                User::Name,
                User::AvatarUrl,
                User::Surname,
                User::Email,
                User::Phone,
                User::Balance,
                User::PasswordHash,
                User::IsAdmin,
            ])
            .values_panic([
                "Test".into(),
                "".into(),
                "Test".into(),
                "Test@Test.com".into(),
                "123456789".into(),
                0.0.into(),
                password_hash.into(),
                true.into(),
            ])
            .to_owned();
    
        manager.exec_stmt(insert).await?;
        */
        
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

        manager
            .drop_table(Table::drop().table(Reviews::Table).to_owned())
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
    CompanyName,
    Email,
    Phone,
    TelegramId,      
    Balance,
    PasswordHash,
    IsAdmin,
    EmailVerified,
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


#[derive(DeriveIden)]

enum Payment {
    Table,
    Id,
    OrderId,
    UserId,
    Amount,
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