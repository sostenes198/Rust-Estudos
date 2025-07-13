use super::traits::file_reader::FileReader;
use std::{fs, io};

pub struct FileReaderSystem;

impl FileReader for FileReaderSystem {
    fn read_to_string(&self, path: &String) -> io::Result<String> {
        fs::read_to_string(path)
    }
}
