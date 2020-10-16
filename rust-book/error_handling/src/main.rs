// Example code from Rust Book Chapter 9
// https://doc.rust-lang.org/book/ch09-00-error-handling.html

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    //println!("Hello, world!");
    println!("Error handling examples");
    //panic_basic()
    //panic_backtrace();
    //result_file();
    //result_file_match();
    //result_file_unwrap();
    result_file_unwrap_shortcut();
}

fn panic_basic() {
    //panic!("crash and burn");
}

fn panic_backtrace() {
    let v = vec![1, 2, 3];

    v[99]; // invalid index!  Oh no!
           // export RUST_BACKTRACE=1 to get a full stack trace
           // or just simply "RUST_BACKTRACE=1 cargo run"
}

fn result_file() {
    let f = File::open("hello.txt");
    // Standard library docs
    // to discover it returns a Result
    // https://doc.rust-lang.org/std/index.html

    //let f: u32 = File::open("hello.txt"); //Wrong type, but guessing
    // // look at the compiler error for hints
}

fn result_file_match() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn result_file_match2() {
    fn main() {
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            },
        };
    }
}

fn result_file_unwrap() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn result_file_unwrap_shortcut() {
    //let f = File::open("hello.txt").unwrap();
    // get value or panic generically!

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    // get value or panic with helpful message.  Better!
}

fn result_propogate_error() {}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_even_shorter() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

//fn result_return_questionmark_() {
//    let f = File::open("hello.txt")?; //oops!  only allowed to use ? if we handle the Result or Option
//}

fn result_return_questionmark() -> Result<(), Box<dyn Error>> {
    // Uhhh, what is a Box?
    let f = File::open("hello.txt")?;

    Ok(())
}
