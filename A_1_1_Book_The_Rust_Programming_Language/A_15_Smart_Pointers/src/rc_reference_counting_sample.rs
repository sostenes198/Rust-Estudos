// enable multiple ownership explicitly by using the Rust type Rc<T>, which is an abbreviation for reference counting.
// The Rc<T> type keeps track of the number of references to a value to determine whether or not the value is still in use.
// If there are zero references to a value, the value can be cleaned up without any references becoming invalid.
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::rc_reference_counting_sample::List::{Cons, Nil};
use std::rc::Rc;

pub fn sample() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
