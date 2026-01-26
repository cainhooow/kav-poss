use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20250904_212116_create_roles_table::Role, m20260125_002042_create_plans_table::Plan};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let mut plans_fk = ForeignKey::create()
            .from(PlanFeaturePivot::Table, PlanFeaturePivot::PlanId)
            .to(Plan::Table, Plan::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();
        let mut feature_fk = ForeignKey::create()
            .from(PlanFeaturePivot::Table, PlanFeaturePivot::FeatureId)
            .to(Role::Table, Role::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(PlanFeaturePivot::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlanFeaturePivot::PlanId)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PlanFeaturePivot::FeatureId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(&mut plans_fk)
                    .foreign_key(&mut feature_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(PlanFeaturePivot::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PlanFeaturePivot {
    Table,
    PlanId,
    FeatureId,
}
