use std::io;
use mockall::automock;

#[automock]
pub trait FileReader {
    fn read_to_string(&self, path: &String) -> io::Result<String>;
}
