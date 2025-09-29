pub use sea_orm_migration::prelude::*;

mod m20250929_063424_create_subscriptions_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            m20250929_063424_create_subscriptions_table::Migration,
        )]
    }
}
