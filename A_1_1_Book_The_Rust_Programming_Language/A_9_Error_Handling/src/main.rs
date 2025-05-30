use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

// Changing main to return Result<(), E> allows the use of the ? operator on Result values.
fn main() -> Result<(), Box<dyn Error>> {
    sample();

    // Won't compile
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

fn sample() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    // let greeting_file_result = File::open("hello.txt");

    // match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(error) => panic!("Problem creating the file: {:?}", error),
    //         },
    //         other_erro => panic!("Problem opening the file: {:?}", other_erro),
    //     },
    // };

    File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the
    // Err variant, unwrap will call the panic! macro for us. Here is an example of unwrap in action:
    let greeting_file = File::open("hello.txt").unwrap();

    // We use expect in the same way as unwrap: to return the file handle or call the panic! macro.
    // The error message used by expect in its call to panic! will be the parameter that we pass to expect, rather than the default panic!
    // message that unwrap uses.
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_simple() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;

    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_simple_1() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_simle_2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
