use std::io;

fn main() {
    println!("Hello, world! ");
    println!("Welcome to the Temperatures conversion between Fahrenheit and Celsius");

    'main_loop: loop {
        println!("Enter 1 for F to C");
        println!("Enter 2 for C to F");
        println!("Enter 0 for quit!");

        let mut opt_one = String::new();
        io::stdin()
            .read_line(&mut opt_one)
            .expect("Failed To read line");
        let opt_one: u8 = match opt_one.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a valid number.");
                continue;
            }
        };

        if opt_one == 0 {
            break 'main_loop;
        } else if opt_one == 1 {
            f_to_c();
        } else {
            c_to_f();
        }
    }

    println!("Program is over! Thank You");
    println!("To use Run this program again");
}

fn f_to_c() {
    println!("Fahrenheit to Celsius Converter");

    'input_loop: loop {
        println!("Enter the temp in F");
        let mut f = String::new();
        io::stdin().read_line(&mut f).expect("Failed to read line");
        let f: f32 = match f.trim().parse() {
            Ok(float) => float,
            Err(_) => {
                println!("Invalid input, please enter a valid number.");
                continue;
            }
        };

        let c: f32 = (f - 32.0) * (5.0 / 9.0);

        println!("{} F is {}C", f, c);

        break;
    }
}

fn c_to_f() {
    println!("Celsius to Fahrenheit Converter");

    'input_loop: loop {
        println!("Enter the temp in C");
        let mut c = String::new();
        io::stdin().read_line(&mut c).expect("Failed to read line");
        let c: f32 = match c.trim().parse() {
            Ok(float) => float,
            Err(_) => {
                println!("Invalid input, please enter a valid number.");
                continue;
            }
        };

        let f: f32 = (c * 9.0 / 5.0) + 32.0;

        println!("{} C is {}F", c, f);
        break;
    }
}
