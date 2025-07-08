pub use sea_orm_migration::prelude::*;

mod m20250707_114910_create_chats_table;
mod m20250708_103257_create_tolkien_lines_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250707_114910_create_chats_table::Migration),
            Box::new(m20250708_103257_create_tolkien_lines_table::Migration),
        ]
    }
}
