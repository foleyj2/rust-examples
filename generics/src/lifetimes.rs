use std::fmt::Display;

pub fn lifetime_main() {
    danglingref();
    find_longer();
    struct_lifetime();
}

fn danglingref() {
    let r;
    // { // dangling reference if we restrict the scope
    let x = 5;
    r = &x;
    // }
    println!("r: {}", r);
}

fn find_longer() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}

//fn longest<'a>(x: &str, y: &str) -> &str {//can't figure out lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

//fn _broken_longest<'a>(x: &str, y: &str) -> &'a str {
//    let result = String::from("really long string");
//    result.as_str()
//}

fn struct_lifetime() {
    let novel = String::from("Call me Ishmael.  Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
}

// // Lifetime Elison
// fn _first_word(s: &str) -> &str {
// // Wait, why doesn't this need lifetime indication like:
// // fn first_word<'a>(s: &'a str) -> &'a str {

//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

impl<'a> ImportantExcerpt<'a> {
    fn _level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn _announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
fn _static_lifetime() {
    let _s: &'static str = "I have a static lifetime.";
    // static can live for the lifetime of the whole program
}

fn _longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
