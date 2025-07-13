// src/main.rs
fn main() {
    let dangling_reference = dangle();
}
fn dangle() -> &String {
    // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, so its memory goes away.
// Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}