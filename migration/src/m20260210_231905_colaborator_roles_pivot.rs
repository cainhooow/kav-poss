use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260209_133913_create_company_roles_table::CompanyRole,
    m20260209_133932_create_company_colaborators_table::CompanyColaborator,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let mut role_fk = ForeignKey::create()
            .from(ColaboratorRole::Table, ColaboratorRole::RoleId)
            .to(CompanyRole::Table, CompanyRole::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();
        let mut colaborator_fk = ForeignKey::create()
            .from(ColaboratorRole::Table, ColaboratorRole::ColaboratorId)
            .to(CompanyColaborator::Table, CompanyColaborator::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(ColaboratorRole::Table)
                    .col(pk_auto(ColaboratorRole::ColaboratorId))
                    .col(ColumnDef::new(ColaboratorRole::RoleId).integer().not_null())
                    .foreign_key(&mut role_fk)
                    .foreign_key(&mut colaborator_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(ColaboratorRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ColaboratorRole {
    Table,
    RoleId,
    ColaboratorId,
}
