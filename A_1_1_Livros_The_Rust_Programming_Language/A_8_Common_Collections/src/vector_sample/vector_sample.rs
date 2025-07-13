pub fn vector_sample() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // let does_not_exist = &v[100];
    // println!("The does_not_exist element is {}", does_not_exist); // panic error

    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(does_no_exist) => println!("The does not exist element is {}", does_no_exist),
        None => println!("There is no doest not exist element"),
    }

    // borrow references error code sample
    // let mut v = vec![1, 2, 3, 4, 5];
    //
    // let first = &mut v[0];
    //
    // v.push(6);
    //
    // println!("The first element is: {first}");


    let mut v = vec![100, 32, 57];
    for i in &mut v {
        println!("{}", i);
        *i += 50;
    }
    for i in &mut v {
        println!("{}", i);
    }

    // Like any other struct, a vector is freed when it goes out of scope, as annotated
    {
        let scopeVec = vec![1, 2, 3, 4];
        // do stuff with scopeVec
    } // <- scopeVec goes out of scope and is freed here
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn sample_spread_sheet_cell() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("BLUE")),
        SpreadsheetCell::Float(10.12),
    ];
}