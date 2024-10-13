use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome everyone \n");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("the secret number is {}", secret_number);

    println!("Guess the Number Game");
    loop{
        println!("Please input the guess:");
        
        let mut guess = String::new();
        // guess.push("Aditya KUmar");

        io::stdin().read_line(&mut guess).expect("Failed to read user input");
        println!("You guessed:  {}", guess);


        // let guess:u32 = guess.trim().parse().expect("Failed to parse!");// Shadowing
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) =>{
                println!("Invalid Input! . Try again");
                continue;
            },
        };

        match guess.cmp(&secret_number){
            Ordering::Equal => {
                println!("You Won!");
                break;
            },
            Ordering::Greater => println!("Too high! Try again.!"),
            Ordering::Less => println!("Too low! Try again."),
        };
    }





    // println!("You guessed:  {guess}");
    // guess.cmp(&secret_number);
    
    // println!("Thanks");
    // println!("Ok Bye");

    
}
