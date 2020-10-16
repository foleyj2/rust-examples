mod lifetimes;
mod traits;

fn main() {
    //test_typed_largest();
    //test_generic_largest();
    //traits::traits_main();
    lifetimes::lifetime_main();
}

fn test_typed_largest() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest: char = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//fn largest<T>(list: &[T]) -> T {
//     // Generic but doesn't work due to need of PartialOrd trait!
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn test_generic_largest() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T1, T2> {
    x: T1,
    y: T2,
}

fn _point_test() {
    //let wont_work = Point { x: 5, y: 4.0 };
    let works = Point2 { x: 5, y: 4.0 };
}

// impl<U> Point<U> {
//     fn x(&self) -> &U {
//         &self.x
//     }
// }

// fn _point_test2() {
//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.x());
// }

// impl Point<f32> {
//     fn _distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
