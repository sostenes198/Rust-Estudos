mod common;

use A_11_Writing_Automated_Tests;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, A_11_Writing_Automated_Tests::add_two(2));
}
