use std::fs::File;
use std::io::ErrorKind;


fn main() {
    println!("Hello, world!");

    // let r = divide(4, 0);
    let r = divide(4, 0).unwrap_or(-1);
    // if succues then go on 
    // if fail then panic

    println!("R = {r:?}");

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}


// fn divide (x:i32 , y:i32) ->i32 {
//     x/y
// }

fn divide (x:i32 , y:i32) -> Result<i32, String> {
    if y == 0{
        return Err(String::from("Please do not divide by zero"));
    }

    Ok(x/y)
}