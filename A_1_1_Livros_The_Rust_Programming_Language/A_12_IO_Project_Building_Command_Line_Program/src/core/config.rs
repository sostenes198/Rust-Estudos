#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        Ok(Config { query, file_path })
    }
}
