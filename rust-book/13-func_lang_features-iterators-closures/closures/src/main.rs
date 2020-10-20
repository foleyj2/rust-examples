use std::thread;
use std::time::Duration;
//13-1
//A function to stand in for a hypothetical calculation that takes about 2 seconds to run
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// 13-2
//  A main function with hardcoded values to simulate user input and random number generation
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout_basic(simulated_user_specified_value, simulated_random_number);
    generate_workout_fn(simulated_user_specified_value, simulated_random_number);
    generate_workout_closure(simulated_user_specified_value, simulated_random_number);
    generate_workout_cacher(simulated_user_specified_value, simulated_random_number);

    closure_syntax();
}

// 13-3
// The business logic that prints the workout plans based on the inputs and calls to the simulated_expensive_calculation function
fn generate_workout_basic(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

// 13-4
// Extracting the calls to simulated_expensive_calculation to one place and storing the result in the expensive_result variable
fn generate_workout_fn(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}

// 13-6
// Calling the expensive_closure we’ve defined
fn generate_workout_closure(intensity: u32, random_number: u32) {
    // 13-5
    //  Defining a closure and storing it in the expensive_closure variable
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // 13-7
    // Adding optional type annotations of the parameter and return value types in the closure
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

#[allow(dead_code)]
fn closure_syntax() {
    fn _add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let _add_one_v2 = |x: u32| -> u32 { x + 1 };
    //let add_one_v3 = |x| { x + 1 }; Needs type annotation and clippy gets rid of braces
    //let add_one_v4 = |x| x + 1; Needs type annotation

    // 13-8
    // Attempting to call a closure whose types are inferred with two different types
    // let example_closure = |x| x;

    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
}

// 13-9
// Defining a Cacher struct that holds a closure in calculation and an optional result in value
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

// 13-10
// The caching logic of Cacher
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// 13-11
// Using Cacher in the generate_workout function to abstract away the caching logic
fn generate_workout_cacher(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
