use std::cmp::Ordering; // std library, cmp method, Ordering type
use std::io; // imports io(input/output) module from std library
use rand::Rng; // we use the rand crate for the Rng trait for... rng

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}"); 

    println!("Please input your guess.");

    let mut guess = String::new(); // create mutable variable 'guess'. it's of type 'String', and it's 'new'.

    io::stdin() // same as 'std::io::stdin', but 'use' already called the io lib from the std library earlier, so we can just use 'io::stdin' now.
    
        .read_line(&mut guess) // calls the 'read_line' method from the 'stdin' fn. '(&mut guess)'
                               // means it's a mutable reference, rather than immutable
                               // '&guess''(&mut guess)' means it's a mutable reference, rather
                               // than immutable reference '&guess'.
                               
        .expect("Failed to read line"); // used to crash the program if the Result returns 'err',
                                        // aka,  // used to crash the program if the Result returns
                                        // 'err'.
                                        
    // note that the above method could have been written as one line, but book has me do it in multiple lines for sake of clarity.

    let guess: u32 = guess.trim().parse().expect("Please type a number!"); 
    // .trim method removes whitespace, necessary for converting string to u32. 
    // -- (hitting 'enter' for the input from read_line creates a newline character and must be trimmed. 
    // ---EXAMPLE: inputting 5 would be "5\n", must trim the "\n".)
    // .parse method is used to convert a string to another type.
    // -- .parse returns a Result type, which we use the expect method to handle again.
    // --- if .parse returns an Err Result variant, it will crash the program with the message provided.
    // the colon in "guess:" tells rust we'll annotate the variable's type.

    println!("You guessed: {guess}");
    
    match guess.cmp(&secret_number) {
    // .cmp method compares two values - in this case, guess and secret_number
    // match expression tells program what to do based on outcome of the Ordering type returned from cmp method
        Ordering::Less => println!("Too small!"), 
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    // match expression ends after first successful match, and stops looking further
    }
}

// "trivial change"
// continue at multiple guesses w/ looping