// using rand crate as external dependency. "call anything in the rand crate by placing rand:: before it"
extern crate rand;

// 'use' brings type not found in the prelude into scope of program
// 'io' (input/output) library from 'std' (standard) library
use std::io;
use std::cmp::Ordering; // like result (another emun) with outcomes of Less / Greater / Equal from comparison of two values
use rand::Rng; // Rng trait defines methods of rand thus in global scope for use in entire program

fn main() { // program entry point
    println!("Guess the number!");

    // rand::thread_rng() is function, grants us random generator we're using
    // "local to the current thread of execution and seeded by the operating system"
    // gen_range() method called on this generator.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    /* creating new mutable string buffer called 'guess'
    which is equal to the return value of the function
    'String::new()' which returns a new string buffer
    */
    let mut guess = String::new();

    /*
    calling associated function `stdin` on `io` lib. Returns an instance
    of `std::io::Stdin` - type represents handle to standard input of
    terminal
    */
    io::stdin().read_line(&mut guess) // method called on input handle to get user input. Passing mut ref to 'guess' as argument. Returns a value - an `io::Result`
        .expect("Failed to read line"); // checking `Ok` / `Err` & returning value entered

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) { 
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
    }

}
