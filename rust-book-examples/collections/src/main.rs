mod hashmaps;
mod strings;

fn main() {
    //println!("Hello, world!");
    //new_vec();
    //update_vec();
    //drop_vec();
    //read_vec();
    //read_nonexist_elem();
    //bad_mut_nonmut_elem();
    //iterate_vec();
    //enum_mult_types();
    strings::main_strings_examples();
    //hashmaps::main_hashmaps_examples();
}

fn new_vec() {
    let _v: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3]; // inferred type
}

fn update_vec() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
}

fn drop_vec() {
    {
        let _v = vec![1, 2, 3, 4];
    } // out of scope, freed!
}

fn read_vec() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn read_nonexist_elem() {
    let v = vec![1, 2, 3, 4, 5];

    //let _nonexist = &v[100]; //causes panic
    let _nonexist2 = v.get(100);
}

fn bad_mut_nonmut_elem() {
    let mut v = vec![1, 2, 3, 4, 5];
    //let _first = &v[0]; //immutable borrow causes problem because vector might change!

    // let first = &mut v[0]; //mutable can only be one at a time.  The push also borrows.
    v.push(6); //as it does here
    let _first = &v[0]; // but we can borrow afterwards
    println!("The first element is: {}", _first);
}

fn iterate_vec() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        println!("before: {}", i);
        *i += 50;
        println!("after: {}", i);
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn enum_mult_types() {
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
