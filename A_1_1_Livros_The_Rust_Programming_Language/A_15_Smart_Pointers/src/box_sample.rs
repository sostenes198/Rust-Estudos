use crate::box_sample::List::Cons;
use crate::box_sample::List::Nil;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn sample() {
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
