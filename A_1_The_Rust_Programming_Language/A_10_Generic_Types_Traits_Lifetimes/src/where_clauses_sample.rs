use std::fmt::{Debug, Display};

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    10
}

fn some_function_where<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    10
}
