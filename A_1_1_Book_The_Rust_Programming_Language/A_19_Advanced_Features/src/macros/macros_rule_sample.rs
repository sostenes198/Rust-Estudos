macro_rules! ss {
 ( $( $x:expr ),* ) => {
 {
   let mut temp_vec = Vec::new();
   $(
    temp_vec.push($x);
   )*
   temp_vec
  }
 };
}

pub fn sample() {
    let x = ss![1, 2, 3, 4, 5];
    println!("{}", x[0]);
}
