pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;

mod m20230921_173921_relations4;
mod m20230921_174343_update1;
mod m20230921_211021_update2;
mod m20230921_213853_update3;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230921_173921_relations4::Migration),
            Box::new(m20230921_174343_update1::Migration),
            Box::new(m20230921_211021_update2::Migration),
            Box::new(m20230921_213853_update3::Migration),
        ]
    }
}
