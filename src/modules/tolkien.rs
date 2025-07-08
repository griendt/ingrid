use crate::entity::tolkien_line;
use crate::modules::Module;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

pub struct Tolkien {}

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
}

impl Module for Tolkien {
    async fn handle(&self, input: String, db: &DatabaseConnection) -> String {
        if input == "seed" {
            self.seed(db).await;
        }

        let line = usize::from_str(input.as_str())
            .ok()
            .or_else(|| {
                File::open("./silmarillion-line.txt")
                    .ok()
                    .and_then(|file| Some(BufReader::new(file)))
                    .and_then(|buffer| Some(buffer.lines().nth(0).unwrap().unwrap()))
                    .and_then(|line| usize::from_str(line.as_str()).ok())
            })
            .unwrap_or(1);

        tolkien_line::Entity::find()
            .filter(tolkien_line::Column::LineNumber.eq(line as i32))
            .one(db)
            .await
            .expect("Could not query the database")
            .and_then(|model| Some(model.line_content))
            .unwrap_or("Deze regel kan ik niet lezen.".to_string())
    }
}
