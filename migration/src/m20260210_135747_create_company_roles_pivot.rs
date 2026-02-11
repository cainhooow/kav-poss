use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250904_212116_create_roles_table::Role, m20260123_222500_create_companies_table::Company,
    m20260209_133913_create_company_roles_table::CompanyRole,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let mut flag_fk = ForeignKey::create()
            .from(CompanyRolePivot::Table, CompanyRolePivot::FlagId)
            .to(Role::Table, Role::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut company_role_fk = ForeignKey::create()
            .from(CompanyRolePivot::Table, CompanyRolePivot::CompanyRoleId)
            .to(CompanyRole::Table, CompanyRole::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(CompanyRolePivot::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CompanyRolePivot::CompanyRoleId)
                            .integer()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyRolePivot::FlagId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(&mut flag_fk)
                    .foreign_key(&mut company_role_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(CompanyRolePivot::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum CompanyRolePivot {
    Table,
    FlagId,
    CompanyRoleId,
}
