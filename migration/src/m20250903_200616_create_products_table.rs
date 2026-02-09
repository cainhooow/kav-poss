use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260123_222500_create_companies_table::Company;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let mut company_fk = ForeignKey::create()
            .from(Product::Table, Product::CompanyId)
            .to(Company::Table, Company::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(pk_auto(Product::Id))
                    .col(ColumnDef::new(Product::Name).string().not_null())
                    .col(ColumnDef::new(Product::Description).string().null())
                    .col(ColumnDef::new(Product::Price).integer().not_null())
                    .col(ColumnDef::new(Product::Barcode).string().null())
                    .col(ColumnDef::new(Product::CompanyId).integer().not_null())
                    .col(
                        ColumnDef::new(Product::Sku)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .foreign_key(&mut company_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
    Name,
    #[sea_orm(iden = "description")]
    Description,
    Price,
    Sku,
    Barcode,
    CompanyId,
}
