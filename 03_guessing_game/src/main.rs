// Note: cargo doc --open will generate and
// open the documentation for the project dependencies.
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Note that we don't specify the variable type explicitly,
    // but the language is strongly typed, the type is derived
    // from the method return type.
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The number: {}", secret_number);

    println!("Please input your guess.");

    loop {
        // By default variables are immutable, for example:
        // let x = 5;
        // `let mut guess` declares mutable variable.
        //
        // String::new() - here new() is "associated" function.
        let mut guess = String::new();

        // the `&mut guess` - here we pass guess by reference.
        io::stdin().read_line(&mut guess)
            // read_line returns io::Result object
            // io::Result is an enum with Ok and Err variants (enum values)
            // when we call Err.expect(...), the app will crash with specified
            // message if he Err result is returned.
            .expect("Failed to read line.");

        // We already have "guess", but we can "shadow" (redefine) the
        // existing variable.
        // The `guess: u32` annotates the variable type, because `parse` can
        // return different types and we need to specify the expected one.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // `continue` goes back to the start of the loop
            Err(_) => continue
        };

        println!("You guessed: {}", guess);

        // match construct is similar to switch case,
        // the below code can be written as this:
        //
        // // `cmp` returns the Order enum
        // Order result = guess.cmp(&secret_number)
        //
        // // now we match the result against patterns
        // match result {
        //     // pattern => code to run
        //     Order::Less => ...,
        //     // the rules inside "match" called "arms"
        //     Order::Greater => ...,
        // }
        //
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
