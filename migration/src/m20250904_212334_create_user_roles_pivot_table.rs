use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20250904_211710_create_users_table::User, m20250904_212116_create_roles_table::Role};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let mut users_fk = ForeignKey::create()
            .from(UserRolesPivot::Table, UserRolesPivot::UserId)
            .to(User::Table, User::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut roles_fk = ForeignKey::create()
            .from(UserRolesPivot::Table, UserRolesPivot::UserId)
            .to(Role::Table, Role::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(UserRolesPivot::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRolesPivot::RoleId)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserRolesPivot::UserId).integer().not_null())
                    .foreign_key(&mut users_fk)
                    .foreign_key(&mut roles_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(UserRolesPivot::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserRolesPivot {
    Table,
    UserId,
    RoleId,
}
