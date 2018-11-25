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

    // println!("The secret number is: {}", secret_number); // useful when writing but not when finished obvs

    loop { // infinite loop of code in {}
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

        // convert guess from implicit string to int for comparison with secret_number
        // "shadowing previous value of guess with a new one": reuse var name
        // `match` method allows for error handling instead of crashing on error as with `expect`
        let guess: u32 = match guess.trim().parse() { // trim method removes whitespace & the '\n' from pressing return to enter guess on keyboard. parse - on strings - parses it into a number. ':' is the type annotation (in this case u32) - also then infered type of secret_number is also u32
            Ok(num) => num, // if .parse() returns a result of Ok (variant) it contians the resultant number: new value of guess
            Err(_) => continue, // if it can't parse the string into a number returns Err variant matching the `Err(_)` pattern. '_' is catchall value. 'Continue' moves the program to iterate again from start of loop (ignores errors and loops back for another guess)
        };

        println!("You guessed: {}", guess);

        /*
        cmp compares two values called as method with argument as comparison.
        takes reference to comparison argument. Returns variant of Ordering enum.
        Match expression uses returned variant to decide what to do.
        */
        match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break; // kills loop
            }
        }
    }
}
