use sea_orm_migration::{prelude::*, schema::*};
use sea_orm_migration::sea_orm;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(pk_auto(Role::Id))
                    .col(ColumnDef::new(Role::Name).unique_key().string().not_null())
                    .col(ColumnDef::new(Role::Description).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Role {
    Table,
    Id,
    Name,
    Description,
}
