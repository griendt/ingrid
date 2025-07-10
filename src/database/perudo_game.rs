use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "perudo_game")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub chat_id: i32,
    pub status: Status,
    pub ruleset: Option<Ruleset>,
}

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveIden, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "status")]
pub enum Status {
    #[sea_orm(string_value = "created")]
    Created,
    #[sea_orm(string_value = "started")]
    Started,
    #[sea_orm(string_value = "finished")]
    Finished,
}

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveIden, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "ruleset")]
pub enum Ruleset {
    #[sea_orm(string_value = "noord")]
    Noord,
    #[sea_orm(string_value = "zuid")]
    Zuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
