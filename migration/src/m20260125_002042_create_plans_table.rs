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
                    .table(Plan::Table)
                    .if_not_exists()
                    .col(pk_auto(Plan::Id))
                    .col(ColumnDef::new(Plan::Name).string().not_null())
                    .col(ColumnDef::new(Plan::Price).integer().not_null())
                    .col(ColumnDef::new(Plan::Description).text().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Plan::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Plan {
    Id,
    Table,
    Name,
    Price,
    Description,
}
