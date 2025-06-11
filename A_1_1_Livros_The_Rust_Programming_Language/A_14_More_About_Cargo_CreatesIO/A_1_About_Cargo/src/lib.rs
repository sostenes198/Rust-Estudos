//! A_14_More_About_Cargo_CreatesIO
//!
//! `A_14_More_About_Cargo_CreatesIO` ` is a collection of utilities to make performing
//! certain calculations more convenient.

mod art;

pub use self::art::kinds::PrimaryColor;
pub use self::art::kinds::SecondaryColor;
pub use self::art::utils::mix;

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = A_14_More_About_Cargo_CreatesIO::add_one(arg);
///
/// assert_eq!(6, answer);
pub fn add_one(x: i32) -> i32 {
    x + 1
}
