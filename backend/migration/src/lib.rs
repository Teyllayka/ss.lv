// pub use sea_orm_migration::prelude::*;

// mod m20220101_000001_create_table;

// mod m20230921_173921_relations4;
// mod m20230921_174343_update1;
// mod m20230921_211021_update2;
// mod m20230921_213853_update3;
// mod m20231005_202202_update4;
// mod m20231005_203657_update5;
// mod m20231008_205049_update6;
// mod m20231009_180620_update7;
// mod m20231017_165401_hz;
// mod m20240502_093115_fix;
// mod m20240502_095822_fix;
// mod m20240502_100525_fix;
// mod m20240502_101324_fix;
// mod m20240521_184220_is_admin;
// mod m20240525_110914_is_admin2;
// mod m20240526_162428_delete_cascade;

// pub struct Migrator;

// #[async_trait::async_trait]
// impl MigratorTrait for Migrator {
//     fn migrations() -> Vec<Box<dyn MigrationTrait>> {
//         vec![
//             Box::new(m20220101_000001_create_table::Migration),
//             Box::new(m20230921_173921_relations4::Migration),
//             Box::new(m20230921_174343_update1::Migration),
//             Box::new(m20230921_211021_update2::Migration),
//             Box::new(m20230921_213853_update3::Migration),
//             Box::new(m20231005_202202_update4::Migration),
//             Box::new(m20231005_203657_update5::Migration),
//             Box::new(m20231008_205049_update6::Migration),
//             Box::new(m20231009_180620_update7::Migration),
//             Box::new(m20231017_165401_hz::Migration),
//             Box::new(m20240502_093115_fix::Migration),
//             Box::new(m20240502_095822_fix::Migration),
//             Box::new(m20240502_100525_fix::Migration),
//             Box::new(m20240502_101324_fix::Migration),
//             Box::new(m20240521_184220_is_admin::Migration),
//             Box::new(m20240525_110914_is_admin2::Migration),
//             Box::new(m20240526_162428_delete_cascade::Migration),
//         ]
//     }
// }

pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
    }
}
