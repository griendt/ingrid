pub mod tolkien;
pub mod help;

pub trait Module {
    fn handle(&self, input: String) -> String;
}