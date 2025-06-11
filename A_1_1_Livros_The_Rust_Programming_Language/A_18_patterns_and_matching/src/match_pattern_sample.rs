fn if_sample() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_sample() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}

fn for_sample() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
}

fn variable_sample() {
    let (x, y, z) = (1, 2, 3);
}

fn function_sample() {
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({x}, {y})");
    }

    let point = (3, 5);
    print_coordinates(&point);
}

fn match_named_variable_sample() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

fn multiple_pattern_sample() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn range_of_values_match_sample() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn destructuring_structs_sample() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    };

    let p = Point { x: 0, y: 7, z: 10 };

    let Point { x: a, y: b, z: c } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    assert_eq!(10, c);

    let Point { x, y, z } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    assert_eq!(10, z);

    let p = Point { x: 0, y: 7, z: 10 };

    match p {
        Point { x, y: 0, z: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y, z } => println!("On the y axis at {y}"),
        Point { x, y, z } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {x}"),
    }
}

fn destructuring_enums_sample() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x dir {x}, in the y dir {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}")
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
    }
}

fn destructuring_structs_and_tuples_sample() {
    struct Point {
        x: i32,
        y: i32,
    };

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

fn ignore_value_in_pattern_sample() {
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {y}");
    }

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    let _x = 5; // Starting a variable name with an underscore to avoid getting unused variable warnings
    let y = 10;

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}

fn match_guard_sample() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn bindings_sample() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 1 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Some other id: {id}"),
    }
}

pub fn sample() {
    if_sample();
    while_sample();
    for_sample();
    variable_sample();
    match_named_variable_sample();
    multiple_pattern_sample();
    range_of_values_match_sample();
    destructuring_structs_sample();
    destructuring_enums_sample();
    destructuring_structs_and_tuples_sample();
    ignore_value_in_pattern_sample();
    match_guard_sample();
    bindings_sample();
}
