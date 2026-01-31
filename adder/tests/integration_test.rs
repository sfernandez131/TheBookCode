use adder::add_two;

use crate::common::setup;

mod common;

#[test]
fn it_adds_two() {
    let result = add_two(2);
    setup();
    assert_eq!(result, 4);
}
