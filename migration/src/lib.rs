pub use sea_orm_migration::prelude::*;
mod m20250903_200616_create_products_table;
mod m20250904_211710_create_users_table;
mod m20250904_212116_create_roles_table;
mod m20250904_212334_create_user_roles_pivot_table;
mod m20250909_005702_inser_roles_tables;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250903_200616_create_products_table::Migration),
            Box::new(m20250904_211710_create_users_table::Migration),
            Box::new(m20250904_212116_create_roles_table::Migration),
            Box::new(m20250909_005702_inser_roles_tables::Migration),
            Box::new(m20250904_212334_create_user_roles_pivot_table::Migration),
        ]
    }
}

