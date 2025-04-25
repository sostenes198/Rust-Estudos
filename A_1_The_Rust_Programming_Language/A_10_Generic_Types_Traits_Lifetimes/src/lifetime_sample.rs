use std::fmt::Display;

pub fn lifetime_sample() {}

// Here, we’ve annotated the lifetime of r with 'a and the lifetime of x with 'b.
// As you can see, the inner 'b block is much smaller than the outer 'a lifetime block.
// At compile time, Rust compares the size of the two lifetimes and sees that r has a lifetime of 'a but that it refers to memory with a lifetime of 'b.
// The program is rejected because 'b is shorter than 'a: the subject of the reference doesn’t live as long as the reference.
// fn lifetime_invalid_scope() {
//     let r; // ---------+-- 'a
//     //          |
//     {
//         //          |
//         let x = 5; // -+-- 'b  |
//         r = &x; //  |       |
//     } // -+       |
//     //          |
//     println!("r: {r}"); //          |
// }

fn lifetime_valid_scope() {
    let x = 5; // ----------+-- 'b
    //           |
    let r = &x; // --+-- 'a  |
    //   |       |

    println!("r: {r}"); //   |       |
    // --+       |
} // ----------+

fn generic_lifetime_in_functions() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

fn generic_lifetime_in_functions_2() {
    let string1 = String::from("long string is long");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    // println!("The longest string is {result}");
}

// fn longest_invalid(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn sample_lifetime_in_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// One special lifetime we need to discuss is 'static, which denotes that the affected reference can live for the entire duration of the program. 
// All string literals have the 'static lifetime, is stored directly in the program’s binary, which is always available.
fn stastic_lifetimes(){
    let s: &'static str = "I have a static lifetime.";
}

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}