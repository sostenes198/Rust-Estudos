use std::{env, process};

use A_12_IO_Project_Building_Command_Line_Program as program;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let args = env::args();

    let file_reader = program::core::file_reader_system::FileReaderSystem {};

    let lib = program::Lib::new(file_reader);

    if let Err(e) = lib.run(args) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
