mod back_of_house;
mod front_of_house;
mod legacy_module_control;

pub fn eat_at_restaurant() {
    legacy_module_control::legacy_module_sample::legacy_module_control();
    
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not
    // allowed to see or modify the seasonal fruit that comes
    // with the meal.
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

fn cook_order() {}

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}

mod customer {
    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    panic!()
}

fn function2() -> IoResult<()> {
    // --snip--
    panic!()
}
