use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    // Inferred to be u32 due to future guess cast/parse
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        // std::io::stdin is a handle to the standard input
        // We read from the handle, putting value into mutable reference of guess
        io::stdin()
            // References are similarly immutable by default
            .read_line(&mut guess)
            // read_line returns a Result enum, which is either
            // Ok or Err.
            // If Result is an Err value, expect causes a crash and
            // Displays the message passed into expect.
            // Otherwise if Result is Ok, will take return value
            .expect("Failed to read line");

        // Shadow guess string to number...
        // Trim removes whitespace and newline char
        // Parse converts string to type, with : type informing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

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
