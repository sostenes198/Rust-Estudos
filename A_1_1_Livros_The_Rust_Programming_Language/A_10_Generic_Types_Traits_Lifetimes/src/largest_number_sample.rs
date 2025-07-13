pub fn largest_number_sample() -> i32 {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    *largest
}

pub fn largest<T>(list: &[T]) -> T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    *largest
}

pub fn samle_largest() {
    println!("{}", largest(&vec![1, 2, 3, 4]));
}
