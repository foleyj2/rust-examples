// https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html
// Programming a Guessing Game
use std::io;

// Set this up with
//  $ cargo new guessing_game
//  I have renamed it to 02-guessing_game for consistency, but that is not a
//  legal cargo crate name.  They can't start with numbers.

fn main() {
    // uncomment which function you want to experiment with
    //main_basic();
    //main_rand();
    main_compare();
}

#[allow(dead_code)]
// Processing a Guess (2-1)
fn main_basic() {
    // Processing a guess
    println!("Guess the number!"); //this macro prints things on the screen

    println!("Please input your guess.");

    let mut guess = String::new();
    // A mutable variable can be changed after assignment.
    // By default, all variables are immutable.

    // We are calling a new() associated function from the String type in the
    //   standard library.  (aka static method)

    io::stdin() //keyboard input
        .read_line(&mut guess) //read chars up to a newline
        .expect("Failed to read line"); //if an error, print this message

    // read_line returns a Result which is an enum can be Ok or Err

    // notice that we can break up long lines

    // &mut guess is a reference to the guess variable so
    //   content can be put into it

    println!("You guessed: {}", guess);
}

use rand::Rng;
#[allow(dead_code)]
// Adding code to generate a random number (2-3)
fn main_rand() {
    println!("Guess the number!"); //this macro prints things on the screen

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

// Handling the possible return values of comparing two numbers (2-4)
use std::cmp::Ordering;

fn main_compare() {
    println!("Guess the number!"); //this macro prints things on the screen

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        // Allowing Multiple Guesses with Looping
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // The guess has to be converted to the right type for comparison
        //
        // let guess: u32 = guess
        //     .trim() // remove whitespace
        //     .parse() // convert string into number
        //     .expect("Please type a number!"); //if Err, give this message

        // Ignoring a non-number guess and asking for another guess
        // instead of crashing the program (2-5)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; //Quitting After a Correct Guess
            }
        }
    }
}
