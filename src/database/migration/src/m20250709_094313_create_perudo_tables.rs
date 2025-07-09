use crate::extension::postgres::Type;
use crate::m20250707_114910_create_chats_table::Chat;
use crate::sea_orm::{EnumIter, Iterable};
use futures::future::TryFutureExt;
use sea_orm_migration::schema::enumeration;
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
            .create_type(
                Type::create()
                    .as_enum("status")
                    .values(Status::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PerudoGame::Table)
                    .if_not_exists()
                    .col(pk_auto(PerudoGame::Id))
                    .col(integer(PerudoGame::ChatId))
                    .col(enumeration(PerudoGame::Status, "status", Status::iter()))
                    .to_owned(),
            )
            .and_then(|_| {
                manager.create_foreign_key(
                    ForeignKey::create()
                        .from_tbl(PerudoGame::Table)
                        .from_col(PerudoGame::ChatId)
                        .to_tbl(Chat::Table)
                        .to_col(Chat::Id)
                        .to_owned(),
                )
            })
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PerudoGame::Table).to_owned())
            .and_then(|_| manager.drop_type(Type::drop().name("status").to_owned()))
            .await
    }
}

#[derive(DeriveIden)]
enum PerudoGame {
    Table,
    Id,
    ChatId,
    Status,
}

#[derive(Iden, EnumIter)]
enum Status {
    Created,
    Started,
    Finished,
}
