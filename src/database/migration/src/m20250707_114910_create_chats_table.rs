use futures::future::TryFutureExt;
use sea_orm_migration::schema::big_integer;
use sea_orm_migration::{
    prelude::*,
    schema::{integer, pk_auto},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chat::Table)
                    .if_not_exists()
                    .col(pk_auto(Chat::Id))
                    .col(big_integer(Chat::TelegramId))
                    .to_owned(),
            )
            .and_then(|_| {
                manager.create_index(
                    Index::create()
                        .table(Chat::Table)
                        .col(Chat::TelegramId)
                        .to_owned(),
                )
            })
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chat::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Chat {
    Table,
    Id,
    TelegramId,
}
