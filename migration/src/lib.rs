pub use sea_orm_migration::prelude::*;

mod m20230830_151845_create_user_table;
mod m20230830_162846_create_workspace_table;
mod m20230830_163103_create_workspace_user_activation_table;
mod m20230830_163103_create_workspace_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230830_151845_create_user_table::Migration),
            Box::new(m20230830_162846_create_workspace_table::Migration),
            Box::new(m20230830_163103_create_workspace_user_table::Migration),
            Box::new(m20230830_163103_create_workspace_user_activation_table::Migration),
        ]
    }
}
