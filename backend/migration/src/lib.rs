pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;

mod m20230921_173921_relations4;
mod m20230921_174343_update1;
mod m20230921_211021_update2;
mod m20230921_213853_update3;
mod m20231005_202202_update4;
mod m20231005_203657_update5;
mod m20231008_205049_update6;
mod m20231009_180620_update7;
mod m20231017_165401_hz;

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
            Box::new(m20231005_202202_update4::Migration),
            Box::new(m20231005_203657_update5::Migration),
            Box::new(m20231008_205049_update6::Migration),
            Box::new(m20231009_180620_update7::Migration),
            Box::new(m20231017_165401_hz::Migration),
        ]
    }
}
