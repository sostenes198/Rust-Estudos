// Rust has a special type named ! that’s known in type theory lingo as the empty type because it has no values.
// We prefer to call it the never type because it stands in the place of the return type when a function will never return.
// Here is an example:
// fn bar() -> ! {
//
// }

pub fn sample() {
    print!("forever ");

    loop {
        print!("and ever ");
        break;
    }
}
