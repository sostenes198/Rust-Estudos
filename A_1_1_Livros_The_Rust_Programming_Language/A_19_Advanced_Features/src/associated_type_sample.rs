struct Counter;

// Associated type
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(10)
    }
}

// Generic Type
// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }

pub fn sample() {}
