use crate::m20250707_114910_create_chats_table::Chat;
use futures::future::TryFutureExt;
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
                    .table(ChatTolkienLineNumber::Table)
                    .if_not_exists()
                    .col(pk_auto(ChatTolkienLineNumber::Id))
                    .col(integer(ChatTolkienLineNumber::ChatId))
                    .col(integer(ChatTolkienLineNumber::LineNumber))
                    .to_owned(),
            )
            .and_then(|_| {
                manager.create_foreign_key(
                    ForeignKey::create()
                        .from_tbl(ChatTolkienLineNumber::Table)
                        .from_col(ChatTolkienLineNumber::ChatId)
                        .to_tbl(Chat::Table)
                        .to_col(Chat::Id)
                        .to_owned(),
                )
            })
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ChatTolkienLineNumber::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ChatTolkienLineNumber {
    Table,
    Id,
    ChatId,
    LineNumber,
}
