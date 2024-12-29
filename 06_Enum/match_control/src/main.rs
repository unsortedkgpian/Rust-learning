// Rust has an extermly powerful control flow construct called match
// Match allow you to compare a value agaist a series of patterns and then execue code based on which pattern matches

// Match power comes from the expressiveness of the patterns and the fact that the compiler confirsms that all possible cases are handled


// match run all condition excute -> which fit first

# [derive(Debug)]
enum UsState{
    Alabama,
    Alaska,

}

enum Coin{
    Penny,
    Nickel,
    Dime(UsState),
    // If enum have value 
    Quarter,

}

fn add(num:i32, num2:Option<i32>) ->i32{
    match num2 {
        Some(i) => num +i, 
        None => num, 
    }
}
// Matches are Exhaustive



// Difference between match and if -> if take only boolen and match can take type condition
fn main() {
    println!("Hello, world!");
    println!("Add Result = {},", add(50, Some(30)));

    let dice_roll = 9;
    match dice_roll {
        3=> prinln!("You got a fancy hat "),
        6 => println!("Your fance hat is removed "),
        // other => println!("Move {} steps,", other)
        _ => println!("Move {dice_roll} steps"),
    }
}

fn value_in_cents(coin:Coin) -> u32{
    match coin{
        Coin::Penny => 1,// arms valid only when all coditions are handled
        Coin::Nickel => {
            println!("This is a Nickel");
            5
        },
        Coin::Dime(UsState::Alaska)=>{
            println!("This Dime Usstate of alaska");
            10
        }
        Coin::Dime(State) => 10,
        // we can access the value of quarter here
        Coin::Quarter => 25,

    }
}
