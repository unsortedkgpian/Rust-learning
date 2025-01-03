fn main() {
    println!("Hello, world!");

    // Error Handling 
    // Two Type
    // 1. Recoverable Error 
    // 2. Unrecoverable Error


    // Recoverable Error
    // -> file not found error -> report the problem and retry the operations

    // Unrecoverable Error -> are always synptoms of bugs, -> trying to access a location beyound the end of an arry and so we want to immediately stop the program


    // Result<T, E> -> recoverable error 
    // panic! -> for unrecoverable error




    // Unrecoverable Errors with panic

    panic!("Crash and burn");
}
