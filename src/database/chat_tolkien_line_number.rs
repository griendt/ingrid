use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "chat_tolkien_line_number")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub chat_id: i32,
    pub line_number: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
