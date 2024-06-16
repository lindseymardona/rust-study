use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Please enter your guess.");

    let mut num_guesses = 0;

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // read_line expects a reference variable, but reference variables are inherently immutable, so we need to explicitly pass &mut guess instead of &guess to ensure it's mutable
            .expect("Failed to read line"); // Expect will crash the program if this fails. You can omit it but it'll raise a warning.

        let guess: u32 = match guess.trim().parse() { // Trim removes whitespace, parse converts the value into what is specified (here, it is u32)
            Ok(num) => num,
            Err(_) => continue, // The underscore is a catch all value, so it will match all Err values no matter what type
        }; // Let is a declaration that does not yield any values, so it ends in a semicolon 

        num_guesses += 1;

        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) { 
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        } // Here, match does not need to end in a semicolon because there is a return value
    }

    println!("It took you {num_guesses} guesses.");
}
