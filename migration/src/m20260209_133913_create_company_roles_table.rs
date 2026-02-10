use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(CompanyRole::Table)
                    .col(pk_auto(CompanyRole::Id))
                    .col(ColumnDef::new(CompanyRole::Name).string().not_null())
                    .col(ColumnDef::new(CompanyRole::Description).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(CompanyRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum CompanyRole {
    Id,
    Table,
    Name,
    Description,
}
