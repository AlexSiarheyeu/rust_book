use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
    
        // Receiving user inputs
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,    
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) { // -> 'cmp' means compare
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Gotcha!");
                break;
            }
        }
    }
}

// Notes
//
// let apples = 5; -> immutable
// let mut bananas = 5; -> mutable