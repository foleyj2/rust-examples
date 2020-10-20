fn main() {
    main_fn_pointer();
    main_closure_or_inline();
}
// Function Pointers

// Listing 19-27 Using the fn type to accept a function pointer as an argument
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main_fn_pointer() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

enum Status {
    Value(u32),
    Stop,
}

fn main_closure_or_inline() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

// Returning Closures
// Problem: you are not allowed to use a function pointer fn as a return type

// fn returns_closure() -> dyn Fn(i32) -> i32 {
//     |x| x + 1
// }

// Solution:  Put it in a box
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
