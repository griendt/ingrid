use crate::database::{chat_tolkien_line_number, tolkien_line};
use crate::modules::Module;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use std::fs::File;
use std::io::Read;

pub struct Tolkien {
    pub chat_id: i32,
}

impl Tolkien {
    async fn seed(&self, db: &DatabaseConnection) {
        let mut file = File::open("./silmarillion.txt").expect("Could not open Silmarillion file");
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect("Could not read Silmarillion file");

        for (line_number, line) in contents.split("\n\n").into_iter().enumerate() {
            tolkien_line::ActiveModel {
                line_number: Set(line_number as i32 + 1),
                line_content: Set(line.to_string()),
                ..Default::default()
            }
            .insert(db)
            .await
            .expect("Could not insert Tolkien line");
        }
    }

    async fn get_paragraph(&self, db: &DatabaseConnection, paragraph_number: i32) -> String {
        tolkien_line::Entity::find()
            .filter(tolkien_line::Column::LineNumber.eq(paragraph_number))
            .one(db)
            .await
            .expect("Could not query the database")
            .and_then(|model| Some(format!("{}. {}", paragraph_number, model.line_content)))
            .unwrap_or("Deze regel kan ik niet lezen".to_string())
    }
}

impl Module for Tolkien {
    async fn handle(&self, input: String, db: &DatabaseConnection) -> String {
        if input == "seed" {
            self.seed(db).await;
            return "Ok".to_string();
        }

        let parsed_input = input.parse::<i32>();
        if parsed_input.is_ok() {
            return self.get_paragraph(db, parsed_input.unwrap()).await;
        }

        let mut current_line = match chat_tolkien_line_number::Entity::find()
            .filter(chat_tolkien_line_number::Column::ChatId.eq(self.chat_id))
            .one(db)
            .await
            .expect("Could not query the database")
        {
            Some(record) => record.into_active_model(),
            None => chat_tolkien_line_number::ActiveModel {
                chat_id: Set(self.chat_id),
                line_number: Set(1),
                ..Default::default()
            },
        };

        let current_line_number = current_line.line_number.clone().unwrap();
        let response = self.get_paragraph(db, current_line_number).await;

        current_line.line_number = Set(current_line_number + 1);
        current_line
            .save(db)
            .await
            .expect("Could not update the Tolkien line number for this chat");

        response
    }
}
