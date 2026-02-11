use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250904_211710_create_users_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let mut user_fk = ForeignKey::create()
            .from(Company::Table, Company::Id)
            .to(User::Table, User::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(Company::Table)
                    .if_not_exists()
                    .col(pk_auto(Company::Id))
                    .col(ColumnDef::new(Company::Name).string().not_null())
                    .col(ColumnDef::new(Company::UserId).integer().not_null())
                    .foreign_key(&mut user_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Company::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Company {
    Table,
    Id,
    UserId,
    Name,
}
