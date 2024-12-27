use std::io;

fn main() {
    println!("Hello, world!");
    //***Variable */
    let x = 5;
    // By default variables are immutable
    println!("The value of of x is :{x}");
    // compile-time error
    let mut y = 34;
    println!("The value of y is {}", y);

    // ******Constants**************
    const PI: u32 = 45;
    // mut is not allowed
    // type must be given
    // const can be decalre in any scopes (scopeIndependent or scope is not valid for const)
    // constasts may set to constant expression not the result of a value during runtime

    const q: i32 = 3;
    const p: i32 = 5;
    const r: i32 = p + q;
    // const r: i32 = p + q +magical();
    // const r: i32 = x +y;
    // let mut number = String::new();
    // io::stdin()
    //     .read_line(&mut number)
    //     .expect("Failed to read line");
    // let number: i32 = number.trim().parse().expect("Failed to convert");
    // const W:i32= q+number;// Error due it fixing it it value on runtime
    println!("The value of r is {r}");









//****************Shadowing*********************************** */
// In mutablitily we cannot change the type of variables
// But in shadowing we can change the type also..
// Rusteceans tell first variables is shadown by second variables
// Why we use shadow 
// There will some condition will occurs when the file net access of variable at one time only so mutting it will help to transform but still immutable

    let apple:i32 = 34;
    // apple = 32;// It is mutating
    // let apple = 4 + apple;
    // let apple:u8 = apple+ 1 ;// type during the computing will also the same
    println!("apples: {}", apple);

    let spaces = "      ";
    // spaces = spaces.len()
    let spaces = spaces.len();
}

fn magical() -> u32 {
    10
}