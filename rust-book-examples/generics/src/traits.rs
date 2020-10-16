use std::fmt::Display;

pub fn traits_main() {
    //testtweet();
    test_article_default();
    //    testtweet2();
    //    largest_test();
}

// Kristjan's question:  enums with traits?
pub enum MyEnum {
    X,
    Y,
}

impl Summary for MyEnum {
    fn summarize(&self) -> String {
        match self {
            MyEnum::X => format!("I'm a MyEnum enum X"),
            MyEnum::Y => format!("I'm a MyEnum enum Y"),
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

//impl Summary for NewsArticle {}//Use default implementation

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn testtweet() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

pub trait Summary2 {
    fn summarize_author(&self) -> String; // needs to be implemented

    fn summarize(&self) -> String {
        // default implementations
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn test_article_default() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

pub struct Tweet2 {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary2 for Tweet2 {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn testtweet2() {
    let tweet = Tweet2 {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

pub fn _notify(item: &impl Summary) {
    // syntactic sugar: <T: Summary>(item: &T)
    println!("Breaking news! {}", item.summarize());
}

fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
    // can't return multiple types with this trait though
}

pub fn _notify_bound<T: Summary>(item: &T) {
    // // Trait Bound Syntax
    // pub fn notify(item1: &impl Summary, item2: &impl Summary) {

    // // Multiple trait bounds
    //  pub fn notify(item: &(impl Summary + Display)) {
    //  pub fn notify<T: Summary + Display>(item: &T) {

    println!("Breaking news! {}", item.summarize());
}

// // Where clauses can make traits clearer
//fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { }

//fn some_function<T, U>(t: &T, u: &U) -> i32
//    where T: Display + Clone,
//          U: Clone + Debug
//{}

// // return types that implement traits
fn _returns_summarizable2() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// fn _broken_returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

fn largest_using_traits<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_test() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_using_traits(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_using_traits(&char_list);
    println!("The largest char is {}", result);
}

// Conditional trait bounds
struct _Pair<T> {
    x: T,
    y: T,
}

impl<T> _Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> _Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
