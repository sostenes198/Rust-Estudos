use std::error::Error;

pub mod core;

use core::config::Config;
use core::traits::file_reader::FileReader;

pub struct Lib {
    file_reader: Box<dyn FileReader>,
}

impl Lib {
    pub fn new(file_reader: impl FileReader + 'static) -> Self {
        let file_reader = Box::new(file_reader);
        Self { file_reader }
    }

    pub fn run(&self, args: &[String]) -> Result<(), Box<dyn Error>> {
        let config = Config::new(&args)?;

        let contents = self.file_reader.read_to_string(&config.file_path)?;

        for line in Self::search(&config.query, &contents) {
            println!("{line}");
        }

        Ok(())
    }

    fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();
        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }

        results
    }
}
