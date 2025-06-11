use crate::garden::vegetable::Asparagus;
mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
