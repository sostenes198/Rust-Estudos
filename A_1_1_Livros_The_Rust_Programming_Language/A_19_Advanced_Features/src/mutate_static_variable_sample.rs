static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn static_sample() {
    println!("value is: {HELLO_WORLD}");

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    add_to_count(3);
}
pub fn sample() {
    static_sample();
}
