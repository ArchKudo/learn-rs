extern crate rand;

// io is a library inside the std library for I/O
use std::io;
// Defines trait, must be in scope for thread_rng to work
use rand::Rng;
// Ordering returns result Less/Greater/Equal
use std::cmp::Ordering;

fn main() {
    // Add a prompt
    println!("Guess a number!");

    // `thread_rng` provide os seeded thread-local random number
    // `gen_range` provides random number between [lower, upper)
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess!");

        // `let` is used to define new variables
        // `::new` is an associated function a.k.a static method
        // associated function are implemented with type rather than instance of the type
        let mut guess = String::new();

        // `read_line` returns either Ok/Err
        // On Ok returns number of bytes read, Err handled by expect
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // Add type annotations to u32
        // trim and parse string as u32
        // NOTE: guess variable is *shadowed*
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // `match` similar to switch(?)
        // `cmp` takes reference
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // break out of loop
                break;
            }
        }
    }
}
