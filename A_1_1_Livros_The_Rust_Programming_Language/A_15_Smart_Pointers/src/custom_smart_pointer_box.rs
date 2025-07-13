use std::mem::drop;
use std::ops::{Deref, Drop};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn sample() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // c.drop() Explicit calls to `drop` are forbidden. Use `std::mem::drop` instead. [E0040]
    drop(c);
    println!("CustomSmartPointers created.");
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
