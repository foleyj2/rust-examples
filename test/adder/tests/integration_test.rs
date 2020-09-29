// https://doc.rust-lang.org/book/ch11-03-test-organization.html
use adder;

//mod common;

// IMPORTANT:  The integration tests will only be run if it passes all the unit tests

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
