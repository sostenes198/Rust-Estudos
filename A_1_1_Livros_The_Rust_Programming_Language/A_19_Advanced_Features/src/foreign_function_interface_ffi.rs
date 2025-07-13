unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn sample() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
