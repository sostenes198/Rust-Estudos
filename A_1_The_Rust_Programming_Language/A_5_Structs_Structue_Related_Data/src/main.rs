mod rectangle_sample;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// struct User2 {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual; // trait struct

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("soso"),
        email: String::from("soso@soso.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    user1.email = String::from("another@example.com");

    println!("{}", user1.email);

    let user2 = User {
        email: String::from("new_user_2@user.com"),
        ..user1
    };

    println!("{}", user2.email);

    println!("{}", user1.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}, {}, {}", black.0, black.1, black.2);
    println!("{}, {}, {}", origin.0, origin.1, origin.2);

    let subject = AlwaysEqual;

    rectangle_sample::execute_rectangle_sample()
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
