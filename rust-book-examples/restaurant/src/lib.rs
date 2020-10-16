//use std::{cmp::Ordering, io}
//use std::io::{self, Write};  // same as std::io, etc.
//use std::collections::* // GLOB!
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house; // in another file

//// Moved to front_of_house.rs
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         fn seat_at_table() {}
//     }
//     mod serving {
//         fn take_order() {}
//         fn serve_order() {}
//     }
// }

pub use crate::front_of_house::hosting; // bring it into scope and re-export
                                        // now other code can call add_to_waitlist()

//use::self::front_of_house::hosting; //relative paths also work

// it is bad practice to bring functions all the way into scope e.g.
//use crate::front_of_house::hosting::add_to_waitlist
// but idiomatic to bring the structures in

pub fn eat_at_restaurant() {
    // abs path
    crate::front_of_house::hosting::add_to_waitlist();

    // rel path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // next line won't compile;  not allowed to see or modify seasonal fruit
    //meal.seasonal_fruit = String::from("blueberries");

    // Enums have all fields public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Once we have use, we have brought elements into scope
    hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); //go to parent
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}
