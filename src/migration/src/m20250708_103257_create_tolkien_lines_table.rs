use futures::future::TryFutureExt;
use sea_orm_migration::{
    prelude::*,
    schema::{integer, pk_auto, text},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TolkienLine::Table)
                    .if_not_exists()
                    .col(pk_auto(TolkienLine::Id))
                    .col(integer(TolkienLine::LineNumber))
                    .col(text(TolkienLine::LineContent))
                    .to_owned(),
            )
            .and_then(|_| {
                manager.create_index(
                    Index::create()
                        .table(TolkienLine::Table)
                        .col(TolkienLine::LineNumber)
                        .to_owned(),
                )
            })
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TolkienLine::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TolkienLine {
    Table,
    Id,
    LineNumber,
    LineContent,
}
