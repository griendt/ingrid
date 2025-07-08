use sea_orm::DatabaseConnection;

pub mod help;
pub mod tolkien;

pub trait Module {
    async fn handle(&self, input: String, db: &DatabaseConnection) -> String;
}
