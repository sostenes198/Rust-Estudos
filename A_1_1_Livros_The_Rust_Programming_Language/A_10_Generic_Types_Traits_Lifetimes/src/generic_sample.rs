struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointGenericParam<T, U> {
    x: T,
    y: U,
}

impl<X1, Y1> PointGenericParam<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointGenericParam<X2, Y2>) -> PointGenericParam<X1, Y2> {
        PointGenericParam {
            x: self.x,
            y: other.y,
        }
    }
}

// enum Option<T> {
//     Some(T),
//     None,
// }
//
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

pub fn generic_sample() {
    let integer: Point<i32> = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let p1 = PointGenericParam { x: 5, y: 10.4 };
    let p2 = PointGenericParam { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
