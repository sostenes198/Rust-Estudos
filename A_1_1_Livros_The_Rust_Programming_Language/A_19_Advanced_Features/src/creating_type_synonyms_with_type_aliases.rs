use std::fmt;
use std::io::Error;

type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
    // --snip- -
}

fn returns_long_type() -> Thunk {
    Box::new(|| {
        println!("hi");
    })
}


// Using trait as-is
//pub trait Write {
    // fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    // fn flush(&mut self) -> Result<(), Error>;
    // 
    // fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    // 
    // fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
//}


// Using trait with  Type Aliases
type Result<T> = std::result::Result<T, std::io::Error>;
pub trait Write{
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

pub fn sample() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| {
        println!("hi");
    });
}
