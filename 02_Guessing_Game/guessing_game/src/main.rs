use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number Game!");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("A secret number has been generated!");

        // Inner loop for guessing
        loop {
            println!("Please input your guess:");

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess_number: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input, please enter a valid number.");
                    continue;
                }
            };

            match guess_number.cmp(&secret_number) {
                Ordering::Equal => {
                    println!("Congratulations, you guessed the correct number!");
                    break;
                }
                Ordering::Less => println!("Too small! Try a bigger number."),
                Ordering::Greater => println!("Too big! Try a smaller number."),
            }
        }

        println!("Enter 0 to exit or 1 to start a new game:");
        let mut opt = String::new();
        io::stdin().read_line(&mut opt).expect("Failed to read line");

        match opt.trim().parse::<u32>() {
            Ok(1) => continue, // Start a new game
            Ok(0) => break,    // Exit the game
            _ => {
                println!("Invalid input. Exiting the game.");
                break;
            }
        }
    }

    println!("Thank you for playing! Run the program again to play more.");
}
