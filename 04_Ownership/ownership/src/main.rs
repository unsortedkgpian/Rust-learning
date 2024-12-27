fn main() {
    println!("Hello, world!");
    // we should now how other langugue do grabage collector
    // Owership is set of rules that govern how a rust program manges memory
    // some have grabage collector or some have explicity allocate and free the memory
    // none of ther features of owenrship will slow down your program while its running the program

    // Main Usp of rust is memory mangangemt system>>

    // The Stack and The Heap
    // It define how onership will work
    // Stack -> all data must have known and fixed size
    // heap -> all data with an unkonw size at compile time or size must change be strore on the heap

    // How heap work
    // heap is less organized
    // memory allocator finds an emplty spot in the heap that is big enough , mark it in use and return POINTERS -> allocating>>>

    let a = String::new();
    // store at heap and return pointer &str

    // accessing the data on heap is slower compare to stack -> follow the pointer

    // When your code calls a function , the values passed into the function (including potentiall, pointer to data on the heap) and the functions local variables get pushed onto the stack, When the function is over those values get popped off the stack

    // keaping tack help to reduce the duplicate data on the heap
    // cleaning of the heap to save space
    // Main purpose of ownership is to manage heap data .

    //**************************************************************************************** */
    // Ownership Rules
    // Each value is Rust has an owner
    // There can only be one owner ata a time
    // When the owner goes out of scope the value will be dropped

    let s = "hello";
    // s is variable and owner is main
    //s is valid from the point at which declared untill the end of the current scope

    // s come into scope , it is valid
    // it remain valid untill it goes out of scope
    // Hardcode string are immutable

    let mut a = "Aditya"; // Hardcode -> stack
    let b = String::from("Aditya"); // ->heap // growable

    let mut v = String::from("Kumar");
    v.push_str(" Raju");
    // a.push_str(" Raju");// Error
    // method not found in `&str`
    println!("V = {v}");

    let g = "motorola"; // string literal // fast but immutable even we use mut

    let kp = String::new();
    // Rust has space funtion drop in-built funtion of rust

    // this methode is called implicitly when the value goes out of scope // automaticla call

    // cannot called explicitly // user can call

    // IN C++ Resource Acquistion Is Initialization (RAII)

    let mut x = 5;
    let y = x;
    println!("The value of x is {x} and y is {y}");
    x = 6;
    println!("The value of x is {x} and y is {y}");

    // stack copy the data

    let pk = String::from(" YOu are gay");

    // *******************************************
    let cd = pk; // Error throw
                 // we think it copy or send the pointer

    // println!("The value of pk is {pk} and cd is {cd}");

    // Stirng in rust is made of three parts
    // 1. pointer -> to the heap
    // 2. len -> length of string
    // 3. capacity -> total amount of memory in byts theat string has recivided

    // store in stack memory all thre thing in stcak memory

    // s1 to s2 we give owernship of that heap momery to S2
    //***Only one owener is there */
    // if there is 2 variable which where pointing to same memory then
    // When goes out of scope it will try to free up the same heap memoty
    // which will create a problems

    // **********Double free Error-> freering memory twice can lead to memory corruption which can leads to security vulnerabilites

    let s1 = String::from("Aditya");
    let s2 = s1;
    // form this poinst s1 is no longer valid

    // In other programing language
    // shallow copy and deep copy
    // concept of coyping -> pointer, length, capacity -. shallow copy
    // Rust also invalidate the variable it is called a move

    // if want to make a copy of string use .clone value

    let pk = String::from("kdjfkjak");
    let ki = pk.clone(); // make a new heap data
    println!("The value of pk is {} and ki = {}", pk, ki);

    // It is only valid for heap data
    // When there is stack data like
    let pj = 233;

    let zds = pj;
    // clone is not required

    // All int boolean floating and char

    // **************************
    // Typles only if its all value have int or bool or float type

    let num: i32 = 24;
    let result: i32 = add(num);
    println!("Num is {num} and result= {result}");
    // num is not change but in case of string or heap memory

    let rt = String::from("Aditya");
    take_ownership(rt);
    // println!("The rt value after function call is {rt}");// Error
    // This trow an error cause it give ownership to take_onwersip funtion

    let poika = gives_ownership();
    println!("The new value of poika is {poika}");

    // giving and taking ownership of the string
    let poika = takes_and_gives_back(poika);
    println!("The new value of poika is {poika}");

    ///  One way to take owership and return is tidus work using touple
    /// 
    /// But using Reference make it easy to give a refresne
    /// 

}

// add
fn add(x: i32) -> i32 {
    x + 10
}

fn take_ownership(str: String) {
    println!("The str input is {str}");
} // drop is called

fn gives_ownership() -> String {
    let s = String::from("Enter pikachu");

    s
}

fn takes_and_gives_back(s: String) -> String {
    println!("Taking the ownership of string s:{s}");
    s
}
