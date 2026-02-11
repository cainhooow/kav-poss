use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260123_222500_create_companies_table::Company;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let mut company_fk = ForeignKey::create()
            .from(CompanyRole::Table, CompanyRole::CompanyId)
            .to(Company::Table, Company::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(CompanyRole::Table)
                    .if_not_exists()
                    .col(pk_auto(CompanyRole::Id))
                    .col(ColumnDef::new(CompanyRole::Name).string().not_null())
                    .col(ColumnDef::new(CompanyRole::Description).string())
                    .col(ColumnDef::new(CompanyRole::CompanyId).integer().not_null())
                    .foreign_key(&mut company_fk)
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
    CompanyId,
}
