use crate::TelegramId;
use crate::database::{perudo_game, perudo_game_player};
use crate::modules::Module;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, Iden, QueryFilter, Set,
};

pub struct Perudo {
    pub chat_id: i32,
    pub from_id: TelegramId,
}

impl Perudo {
    async fn get_game(&self, db: &DatabaseConnection) {}

    async fn create_game(&self, db: &DatabaseConnection) -> Result<perudo_game::Model, DbErr> {
        perudo_game::ActiveModel {
            chat_id: Set(self.chat_id),
            status: Set(perudo_game::Status::Created),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    async fn get_open_game(&self, db: &DatabaseConnection) -> Option<perudo_game::Model> {
        perudo_game::Entity::find()
            .filter(perudo_game::Column::ChatId.eq(self.chat_id))
            .filter(perudo_game::Column::Status.ne(perudo_game::Status::Finished))
            .one(db)
            .await
            .expect("Could not query the database")
    }

    async fn join_game(&self, db: &DatabaseConnection, game: perudo_game::Model) {
        perudo_game_player::ActiveModel {
            perudo_game_id: Set(game.id),
            player_id: Set(self.from_id),
            ..Default::default()
        }
        .insert(db)
        .await
        .expect("Could not join the game");
    }
}

impl Module for Perudo {
    async fn handle(&self, input: String, db: &DatabaseConnection) -> String {
        let mut game: Option<perudo_game::Model> = None;

        match input.as_str() {
            "" => {
                "Met dit commando kun je Perudo spelen! Dit zijn de beschikbare subcommands:\n\
                \n`/perudo create`: Maak een nieuw spel aan in deze chat\
                \n`/perudo join`: Word deelnemer van het spel in deze chat\
                \n`/perudo leave`: Verlaat het spel in deze chat\
                \n`/perudo start`: Start het spel\
                \n`/perudo stop`: Stop het spel vroegtijdig".to_string()
            }
            "create" => match self.get_open_game(db).await.is_some() {
                true => "Er bestaat al een open spel, rond deze eerst af of stuur `/perudo kill` om het spel af te kappen.".to_string(),
                false => {
                    game = Some(self.create_game(db).await.expect("Could not create game"));
                    "Ik heb een nieuw spel aangemaakt! Doe mee door `/perudo join` te sturen. Als iedereen er is, start het spel door `/perudo start` te sturen.".to_string()
                }
            }
            "join" => match self.get_open_game(db).await {
                Some(game) => match game.status {
                    perudo_game::Status::Created => {
                        self.join_game(db, game).await;
                        "Je bent aan het spel toegevoegd!".to_string()
                    }
                    perudo_game::Status::Started => "Sorry, het spel is al gestart. Wacht tot het spel is afgelopen en start dan een nieuwe.".to_string(),
                    perudo_game::Status::Finished => unreachable!(),
                }
                None => "Er is nog geen spel aangemaakt. Gebruik `/perudo create` om een nieuw spel aan te maken.".to_string(),
            }
            _ => unimplemented!(),
        }
    }
}
