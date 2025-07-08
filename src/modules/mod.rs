use sea_orm::DatabaseConnection;

pub mod help;
pub mod tolkien;

pub trait Module {
    fn handle(&self, input: String, db: &DatabaseConnection) -> String;
}
