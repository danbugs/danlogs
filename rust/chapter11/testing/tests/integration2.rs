use testing;
// cargo new testing --lib

mod common;

#[test]
fn test_print_stuff_again_again() {
    common::setup();
    testing::print_stuff()
}