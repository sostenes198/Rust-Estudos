fn main() {
    let s1 = String::from("hello");
    let mut s2 = s1;
    s2.push_str(", world!");

    takes_ownership(s2); // s's value moves into the function...
    // ... and so is no longer valid here

    // println!("{s2}"); // Value used after being moved [E0382]

    let x: i32 = 5;

    make_copy(x);

    println!("{x}");

    let s3 = gives_ownership();
    let mut s4 = takes_and_give_back(s3);
    s4 = takes_and_give_back(s4);

    // println!("{s3}");  // Value used after being moved [E0382]

    println!("{s4}");

    let s5 = String::from("hello");
    let length = calculate_length(&s5);

    println!("The length of '{s5}' is {length}.");
    println!("Memory Address S5 '{}", s5);

    let mut s6 = String::from("hello");
    change(&mut s6);

    println!("{s6}");

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");

    // let reference_to_nothing = dangle();
}

fn takes_ownership(some_string: String) {
    print!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn make_copy(some_interger: i32) {
    println!("{some_interger}");
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_give_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String

    let length = s.len();
    length
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, the String is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }
