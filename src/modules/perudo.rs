use crate::TelegramId;
use crate::database::perudo_game::Ruleset::{Noord, Zuid};
use crate::database::{perudo_game, perudo_game_player};
use crate::modules::Module;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, Iden, IntoActiveModel,
    QueryFilter, Set,
};
use teloxide::types::User;

pub struct Perudo {
    pub chat_id: i32,
    pub from: User,
}

impl Perudo {
    const NO_GAME_FOUND: &'static str = "Er is momenteel geen spel gaande. Gebruik `/perudo create` om een nieuw spel aan te maken.";

    async fn create_game(&self, db: &DatabaseConnection) -> &str {
        perudo_game::ActiveModel {
            chat_id: Set(self.chat_id),
            status: Set(perudo_game::Status::Created),
            ..Default::default()
        }
        .insert(db)
        .await
        .expect("Could not create game");

        "Ik heb een nieuw spel aangemaakt! Doe mee door `/perudo join` te sturen. Als iedereen er is, start het spel door `/perudo start` te sturen."
    }

    async fn get_open_game(&self, db: &DatabaseConnection) -> Option<perudo_game::Model> {
        perudo_game::Entity::find()
            .filter(perudo_game::Column::ChatId.eq(self.chat_id))
            .filter(perudo_game::Column::Status.ne(perudo_game::Status::Finished))
            .one(db)
            .await
            .expect("Could not query the database")
    }

    async fn get_game_status(&self, db: &DatabaseConnection, game: perudo_game::Model) -> String {
        let names = perudo_game_player::Entity::find()
            .filter(perudo_game_player::Column::PerudoGameId.eq(game.id))
            .all(db)
            .await
            .expect("Could not query the database")
            .iter()
            .map(|player| player.player_name.to_owned())
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "Dit is spel ID {}.\
            \nStatus: {}\
            \nRegelset: {}\
            \nSpelers: {}",
            game.id,
            game.status.to_string(),
            match game.ruleset {
                Some(ruleset) => ruleset.to_string(),
                None => "<onbepaald>".to_string(),
            },
            match names.len() {
                0 => "<niemand>".to_string(),
                _ => names,
            }
        )
    }

    async fn join_game(&self, db: &DatabaseConnection, game: perudo_game::Model) -> &str {
        if perudo_game_player::Entity::find()
            .filter(perudo_game_player::Column::PerudoGameId.eq(game.id))
            .filter(perudo_game_player::Column::PlayerId.eq(self.from.id.0))
            .one(db)
            .await
            .expect("Could not query the database")
            .is_some()
        {
            "Je bent al een speler van dit spel."
        } else {
            perudo_game_player::ActiveModel {
                perudo_game_id: Set(game.id),
                player_id: Set(self.from.id.0 as TelegramId),
                player_name: Set(self.from.first_name.to_owned()),
                ..Default::default()
            }
            .insert(db)
            .await
            .expect("Could not join the game");
            "Je bent aan het spel toegevoegd."
        }
    }

    async fn set_ruleset(
        &self,
        db: &DatabaseConnection,
        game: perudo_game::Model,
        input: String,
    ) -> &str {
        let mut game = game.into_active_model();
        let ruleset = match input.split(" ").nth(1) {
            Some("noord") => Noord,
            Some("zuid") => Zuid,
            _ => return "Ik begrijp niet welke regelset je wil. Gebruik `noord` of `zuid`.",
        };

        game.ruleset = Set(Some(ruleset));
        match game.save(db).await {
            Ok(_) => "Ik heb de regelset aangepast.",
            Err(_) => {
                "Het is me niet gelukt de regelset aan te passen. Probeer het later nog eens."
            }
        }
    }

    async fn leave_game(&self, db: &DatabaseConnection, game: perudo_game::Model) -> &str {
        match perudo_game_player::Entity::delete_many()
            .filter(perudo_game_player::Column::PerudoGameId.eq(game.id))
            .filter(perudo_game_player::Column::PlayerId.eq(self.from.id.0))
            .exec(db)
            .await
            .expect("Could not query the database")
            .rows_affected
        {
            0 => "Je was geen onderdeel van het spel.",
            _ => "Je bent uit het spel verwijderd.",
        }
    }
}

impl Module for Perudo {
    async fn handle(&self, input: String, db: &DatabaseConnection) -> String {
        match input.split(" ").nth(0).unwrap_or("") {
            "" => {
                "Met dit commando kun je Perudo spelen! Dit zijn de beschikbare subcommands:\n\
                \n`/perudo status`: De status van het huidige Perudo-spel in deze chat\
                \n`/perudo create`: Maak een nieuw spel aan in deze chat\
                \n`/perudo ruleset <ruleset>`: Stel de te gebruiken regelset in (`noord` of `zuid`)\
                \n`/perudo join`: Word deelnemer van het spel in deze chat\
                \n`/perudo leave`: Verlaat het spel in deze chat\
                \n`/perudo start`: Start het spel\
                \n`/perudo stop`: Stop het spel vroegtijdig".to_string()
            }
            "status" => match self.get_open_game(db).await {
                Some(game) => self.get_game_status(db, game).await,
                None => Self::NO_GAME_FOUND.to_string()
            }
            "create" => match self.get_open_game(db).await.is_some() {
                false => self.create_game(db).await.to_string(),
                true => "Er bestaat al een open spel, rond deze eerst af of stuur `/perudo kill` om het spel af te kappen.".to_string(),
            },
            "ruleset" => match self.get_open_game(db).await {
                Some(game) => self.set_ruleset(db, game, input).await.to_string(),
                None => Self::NO_GAME_FOUND.to_string()
            }
            "join" => match self.get_open_game(db).await {
                Some(game) => match game.status {
                    perudo_game::Status::Created => self.join_game(db, game).await.to_string(),
                    perudo_game::Status::Started => "Sorry, het spel is al gestart. Wacht tot het spel is afgelopen en start dan een nieuwe.".to_string(),
                    perudo_game::Status::Finished => unreachable!(),
                }
                None => Self::NO_GAME_FOUND.to_string()
            },
            "leave" => match self.get_open_game(db).await {
                Some(game) => match game.status {
                    perudo_game::Status::Created => self.leave_game(db, game).await.to_string(),
                    perudo_game::Status::Started => "Sorry, het spel is al gestart. Je kan het spel niet meer verlaten.".to_string(),
                    perudo_game::Status::Finished => "Het spel is al afgelopen.".to_string(),
                },
                None => "Er is geen spel gaande om uit te stappen.".to_string(),
            }
            "start" => match self.get_open_game(db).await {
                Some(game) => match game.status {
                    perudo_game::Status::Created => "Ik ga nu het spel starten!".to_string(),
                    perudo_game::Status::Started => "Het spel is al bezig!".to_string(),
                    perudo_game::Status::Finished => "Het spel is al afgelopen".to_string(),
                },
                None => Self::NO_GAME_FOUND.to_string(),
            }
            _ => unimplemented!(),
        }
    }
}
