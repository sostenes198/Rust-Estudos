use std::ptr::null;

fn main() {
    let word = String::from("Hello, world!");

    let first_word = first_word(&word);

    println!("{first_word}");

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello}");
    println!("{world}");

    let s = String::from("hello");

    let slice = &s[0..2];

    println!("{slice}");

    let slice = &s[..2];

    println!("{slice}");

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    println!("{slice}");

    let slice = &s[3..];

    println!("{slice}");

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    println!("{slice}");
    let slice = &s[..];
    println!("{slice}");

    let mut word = String::from("Hello, world!");

    let first_word = crate::first_word_slice(&word);

    // s.clear(); error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("{first_word}");

    let mut a = [1, 2, 3, 4, 5];

    let mut slice = &a[1..3];

    println!("{}", slice[0]);

    assert_eq!(slice, &[2, 3]);

    slice = &[0, 0];
    println!("{}", a[0]);

    println!("{}", slice[0]);

    a = [0, 0, 0, 0, 0];

    println!("{}", a[0]);
}

fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
