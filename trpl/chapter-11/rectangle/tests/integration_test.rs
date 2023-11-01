use rectangle;

mod common;

#[test]
fn it_adds_two() {
    common::hello();
    assert_eq!(4, rectangle::add_two(2));
}