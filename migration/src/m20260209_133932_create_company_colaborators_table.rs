use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250904_211710_create_users_table::User, m20260123_222500_create_companies_table::Company,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let mut company_fk = ForeignKey::create()
            .from(CompanyColaborator::Table, CompanyColaborator::CompanyId)
            .to(Company::Table, Company::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut user_fk = ForeignKey::create()
            .from(CompanyColaborator::Table, CompanyColaborator::UserId)
            .to(User::Table, User::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(CompanyColaborator::Table)
                    .if_not_exists()
                    .col(pk_auto(CompanyColaborator::Id))
                    .col(
                        ColumnDef::new(CompanyColaborator::CompanyId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyColaborator::UserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyColaborator::Document)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyColaborator::Badge)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(&mut company_fk)
                    .foreign_key(&mut user_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(CompanyColaborator::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum CompanyColaborator {
    Id,
    Table,
    CompanyId,
    UserId,
    Document,
    Badge,
}
