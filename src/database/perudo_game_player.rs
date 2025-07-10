use crate::TelegramId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "perudo_game_player")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub perudo_game_id: i32,
    pub player_id: TelegramId,
    pub player_name: String,
    pub num_dice: i32,
    pub current_dice_roll: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
