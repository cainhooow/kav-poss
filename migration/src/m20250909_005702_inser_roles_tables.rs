use core_server::RoleEnum;
use sea_orm::{entity::*, query::*};
use sea_orm_migration::{prelude::*, schema::*};
use strum::IntoEnumIterator;

use crate::m20250904_212116_create_roles_table::Role;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        for role in RoleEnum::iter() {
            let stmt = Query::insert()
                .into_table(Role::Table)
                .columns([Role::Name])
                .values_panic([role.to_string().into()])
                .to_owned();

            manager.exec_stmt(stmt).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        Ok(())
    }
}
