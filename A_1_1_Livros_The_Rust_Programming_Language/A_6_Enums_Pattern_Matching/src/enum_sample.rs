struct Ipv4Addr {}

struct Ipv6Addr {}

enum IpAddrKind {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

pub fn execute_enum_sample() {
    let four = IpAddrKind::V4(Ipv4Addr {});
    let six = IpAddrKind::V6(Ipv6Addr {});

    route(four);
    route(six);

    // println!("{}", four)

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}

fn route(ip_type: IpAddrKind) {}
