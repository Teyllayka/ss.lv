pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240922_131122_refresh_token;
mod m20240927_210656_payment;





pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240922_131122_refresh_token::Migration),
            Box::new(m20240927_210656_payment::Migration),
        ]
    }
}
