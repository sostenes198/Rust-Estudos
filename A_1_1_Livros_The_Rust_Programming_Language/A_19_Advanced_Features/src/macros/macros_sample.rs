use super::macros_rule_sample;

/// We’ve used macros like println! throughout this book, but we haven’t fully explored what a macro is and how it works.
/// The term macro refers to a family of features in Rust: declarative macros with macro_rules! and three kinds of procedural macros:
///     Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
///     Attribute-like macros that define custom attributes usable on any item
///     Function-like macros that look like function calls but operate on the tokens specified as their argument
///
/// Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming.
/// In Appendix C, we discuss the derive attribute, which generates an implementation of various traits for you.
/// We’ve also used the println! and vec! macros throughout the book. All of these macros expand to produce
/// more code than the code you’ve written manually.
///
/// A function signature must declare the number and type of parameters the function has.
/// Macros, on the other hand, can take a variable number of parameters: we can call println!("hello")
/// with one argument or println!("hello {}", name) with two arguments.
/// Also, macros are expanded before the compiler interprets the meaning of the code, so a macro can, for example,
/// implement a trait on a given type.
/// A function can’t, because it gets called at runtime and a trait needs to be implemented at compile time.

pub fn sample() {
    macros_rule_sample::sample();
}
