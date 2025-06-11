// Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds.
// So what makes RefCell<T> different from a type like Box<T>?
// Recall the borrowing rules
//  At any given time, you can have either one mutable reference or any number of immutable references (but not both).
//  References must always be valid.

// With references and Box<T>, the borrowing rules’ invariants are enforced at compile time.
// With RefCell<T>, these invariants are enforced at runtime.
// With references, if you break these rules, you’ll get a compiler error.
// With RefCell<T>, if you break these rules, your program will panic and exit.

use std::cell::RefCell;
use std::rc::Rc;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent: You're at 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You're at 75% of your quota!");
        }
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::ref_cell_sample::List::{Cons, Nil};

pub fn sample() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));

            // Invalid borrow_mut
            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut();
            //
            // one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
