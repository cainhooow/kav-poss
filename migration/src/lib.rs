pub use sea_orm_migration::prelude::*;
mod m20250903_200616_create_products_table;
mod m20250904_211710_create_users_table;
mod m20250904_212116_create_roles_table;
mod m20250904_212334_create_user_roles_pivot_table;
mod m20250909_005702_insert_roles_tables;
mod m20260123_222500_create_companies_table;
mod m20260125_002042_create_plans_table;
mod m20260125_005629_create_plan_features_pivot_table;
mod m20260209_133913_create_company_roles_table;
mod m20260209_133932_create_company_colaborators_table;
mod m20260210_135747_create_company_roles_pivot;
mod m20260210_231905_colaborator_roles_pivot;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250903_200616_create_products_table::Migration),
            Box::new(m20250904_211710_create_users_table::Migration),
            Box::new(m20250904_212116_create_roles_table::Migration),
            Box::new(m20250904_212334_create_user_roles_pivot_table::Migration),
            Box::new(m20250909_005702_insert_roles_tables::Migration),
            Box::new(m20260123_222500_create_companies_table::Migration),
            Box::new(m20260125_002042_create_plans_table::Migration),
            Box::new(m20260125_005629_create_plan_features_pivot_table::Migration),
            Box::new(m20260209_133913_create_company_roles_table::Migration),
            Box::new(m20260209_133932_create_company_colaborators_table::Migration),
            Box::new(m20260210_135747_create_company_roles_pivot::Migration),
            Box::new(m20260210_231905_colaborator_roles_pivot::Migration),
        ]
    }
}
