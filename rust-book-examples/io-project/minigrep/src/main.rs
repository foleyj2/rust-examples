// https://doc.rust-lang.org/book/ch12-00-an-io-project.html
use std::env; //for args

// Usage: cargo run searchstring example-filename.txt

fn main() {
    //main_read_args();
    //main_save_args();
    //main_read_file();
    //main_extract_arg_parser();
    //main_group_config_vals();
    //main_group_config_constructor();
    //main_extracted();
    //main_handle_returned_errors();
    //main_crate();
    //main_test_driven_dev();
    //main_icase_env_var();
    main_stderr();
}

//////////////////  12.1 Accepting Command Line Arguments
// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html

// 12-1
#[allow(dead_code)]
fn main_read_args() {
    let args: Vec<String> = env::args().collect();
    //args needs to watchout for invalid unicode!
    println!("{:?}", args);
}

// 12-2
#[allow(dead_code)]
fn main_save_args() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}

////////////////// 12.2 Reading a File
// https://doc.rust-lang.org/book/ch12-02-reading-a-file.html

// 12-4
// cargo run the poem.txt
use std::fs;

#[allow(dead_code)]
fn main_read_file() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
    // issue:  each function should be responsible for one idea
    // issue:  we are also not handling errors
}

/////////////////// 12.3 Refactoring to Improve Modularity and Error Handling
// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html
// 12-5

// Separation of Concerns for Binary Projects
#[allow(dead_code)]
fn main_extract_arg_parser() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);
    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

#[allow(dead_code)]
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}

// 12-6
// Grouping config values by improving parse_config
#[allow(dead_code)]
fn main_group_config_vals() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config_improved(&args); //different return type
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config_improved(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}

use std::process;
// 12-7
// Config constructor
#[allow(dead_code)]
fn main_group_config_constructor() {
    let args: Vec<String> = env::args().collect();

    //let config = Config::new(&args);// unhandled panic
    //let config = Config::new_panic(&args); // demonstrate smarter panic

    // 12-10
    let config = Config::new_result(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    }); //BEST!  Return a Result instead

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

#[allow(dead_code)]
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

// 12-8
// Improving error handling
// Panic:  cargo run
#[allow(dead_code)]
impl Config {
    fn new_panic(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

// 12-9
// Even better, returning a Result instead of panic!
#[allow(dead_code)]
impl Config {
    fn new_result(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// 12-11
// Extract logic from main
// We just want to put setup there
#[allow(dead_code)]
fn main_extracted() {
    let args: Vec<String> = env::args().collect();

    //let config = Config::new(&args);// unhandled panic
    //let config = Config::new_panic(&args); // demonstrate smarter panic
    let config = Config::new_result(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    }); //BEST!  Return a Result instead

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

#[allow(dead_code)]
fn run(config: Config) {
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// 12-12
// Changing run() to return  Result
use std::error::Error;

#[allow(dead_code)]
fn run_result(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

#[allow(dead_code)]
fn main_handle_returned_errors() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new_result(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // run_result(config); // gives warning of unhandled errors
    if let Err(e) = run_result(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

//12-14
// moving library functions into lib.rs
use minigrep::Config2;

#[allow(dead_code)]
fn main_crate() {
    let args: Vec<String> = env::args().collect();

    let config = Config2::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

// Developing the Library’s Functionality with Test-Driven Development
// https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html

//12-15 setup
// Test Driven Development needs different main
#[allow(dead_code)]
fn main_test_driven_dev() {
    let args: Vec<String> = env::args().collect();

    let config = Config2::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        //incomplete
        //if let Err(e) = minigrep::run_finished(config) {
        //complete
        println!("Application error: {}", e);

        process::exit(1);
    }
}

/////// Working with Environment Variables
// https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html
use minigrep::ConfigIcase;

// cargo run to poem.txt
// CASE_INSENSITIVE=1 cargo run to poem.txt

#[allow(dead_code)]
fn main_icase_env_var() {
    let args: Vec<String> = env::args().collect();

    let config = ConfigIcase::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run_case_insensitive(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

// Writing Error Messages to Standard Error Instead of Standard Output
// https://doc.rust-lang.org/book/ch12-06-writing-to-stderr-instead-of-stdout.html

// cargo run > output.txt  // can't see error messages!  They end up in the file.

// replace println!  with eprintln!

// cargo run to poem.txt > output.txt // now streams are separated

#[allow(dead_code)]
fn main_stderr() {
    let args: Vec<String> = env::args().collect();

    let config = ConfigIcase::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err); //The magic happens here!
        process::exit(1);
    });

    if let Err(e) = minigrep::run_case_insensitive(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
