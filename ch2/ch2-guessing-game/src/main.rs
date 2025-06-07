use std::io; // imports io(input/output) library from std library

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}"); 

    println!("Please input your guess.");

    let mut guess = String::new(); // create mutable variable 'guess'. it's of type 'String', and it's 'new'.

    io::stdin() // same as 'std::io::stdin', but 'use' on line 1 already called the io lib from the std library earlier, so we can just use 'io::stdin' now.
    
        .read_line(&mut guess) // calls the 'read_line' method from the 'stdin' fn. '(&mut guess)'
                               // means it's a mutable reference, rather than immutable
                               // '&guess''(&mut guess)' means it's a mutable reference, rather
                               // than immutable reference '&guess'.
                               
        .expect("Failed to read line"); // used to crash the program if the Result returns 'err',
                                        // aka,  // used to crash the program if the Result returns
                                        // 'err'.
                                        
    // note that the above method could have been written as one line, but book has me do it in multiple lines for sake of clarity.

    println!("You guessed: {guess}");
}

// "trivial change"
// leaving off at Listing 2-4, continue there.