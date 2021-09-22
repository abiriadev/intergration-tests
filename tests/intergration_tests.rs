use intergration_tests;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(intergration_tests::add_two(10), 12);
}
