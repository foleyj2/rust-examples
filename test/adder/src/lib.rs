// https://doc.rust-lang.org/book/ch11-01-writing-tests.html
#[cfg(test)]
mod test_basic {
    // The free test you get when you create a --lib
    //#[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // FAILING TEST
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// // BUGGY!
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width < other.width && self.height > other.height
//     }
// }

mod test_rectangle {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// // BUGGY!
// pub fn add_two(a: i32) -> i32 {
//     a + 3
// }

#[cfg(test)]
mod test_add_two {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

// // BUGGY!
// pub fn greeting(name: &str) -> String {
//     String::from("Hello!")
// }

#[cfg(test)]
mod test_greeting {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    fn greeting_contains_name_custom() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        // // BUGGY!  CAN PANIC!
        // if value < 1 {
        //     panic!("Guess value must be between 1 and 100, got {}.", value);
        // }

        Guess { value }
    }
}

#[cfg(test)]
mod test_guess {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}

pub struct Guess2 {
    value: i32,
}

impl Guess2 {
    pub fn new(value: i32) -> Guess2 {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        // // BUGGY!  We swapped them
        // if value < 1 {
        //     panic!(
        //         "Guess value must be greater than or equal to 1, got {}.",
        //         value
        //     );
        // } else if value > 100 {
        //     panic!(
        //         "Guess value must be less than or equal to 100, got {}.",
        //         value
        //     );
        // }

        Guess2 { value }
    }
}

#[cfg(test)]
mod test_guess2 {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess2::new(200);
    }
}

#[cfg(test)]
mod test_result {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

// https://doc.rust-lang.org/book/ch11-02-running-tests.html
// You can limit threads
//$ cargo test -- --test-threads=1

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod test_func_output {
    //cargo test -- --show-output

    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(5, value);
    // }
}

// run tests by name (pattern match, really)
// cargo test one_hundred
// cargo test add

#[cfg(test)]
mod test_by_name {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}

// This says don't do the test unless explicitly called or:
// cargo test -- --ignored
mod test_ignored {
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
