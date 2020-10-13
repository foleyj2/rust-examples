// 12-14
// splitting code to a library crate
use std::error::Error;
use std::fs;

pub struct Config2 {
    pub query: String,
    pub filename: String,
}

impl Config2 {
    pub fn new(args: &[String]) -> Result<Config2, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config2 { query, filename })
    }
}

pub fn run(config: Config2) -> Result<(), Box<dyn Error>> {
    let _contents = fs::read_to_string(config.filename)?;

    // 12-15 comment out for TDD
    //println!("With text:\n{}", contents);

    Ok(())
}

//12-15
// a failing test, but it won't even compile at the moment
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result_stub() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_stub(query, contents)
        );
    }
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

//12-16
// stub to allow it to compile
// notice we have to give it a lifetime
#[allow(unused_variables)]
pub fn search_stub<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

//12-17
// Create a useful function
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        //iterator!
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// COMPLETE THE RUN FUNCTION!
pub fn run_finished(config: Config2) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}
