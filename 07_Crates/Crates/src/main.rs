// A pakage can contain multiple binary crates and optionally one library crate

// The nested context in which code is written has a set of names that are define as "in scope", when reading , writing and compilling code , programmers and compilers nedd to know wheter a particular naem ata a particular spot refert to a varable, function, struct , enum, module, constant  or other item and what that item means, 

// Module system-> code's orgnization , details exposed, details private, names in scope

// Packages-> A cargo feature that lets you build, test and share crates
// Crates-> A tree of modules that produces a library or executable
// Modules and use -> Let you control the organization, scope and privacy of paths
// Paths -> A way of naming an item, such as a struct funtion or module





// Pakages and Crates
// Crate is the smallest amount of code that the rust compiler considers at a time -> main.rs
// Crate -> Binary & Library

// Binary Crate -> you can compile to an executable guess the numer -> main function nessery
// Library Crates -> dont have main function -> define fuctionalitiy like rand 

// Crate root is a source file that the rust compiler start from makes up the root module of your crate 



// Package -> A bundle fo one or more crates that provides a set of duvtinality 
// Cargo is a package that contains the binary crate for the command line tool yoy have been using to build your code


// Crate can come in one of two forms: a binary crate or a ibraray crate 

// Package can contain as many binary crates as you like, but most only one library crate 
// min one crate


// src/main.rs -> crate root of a binary crate 
// -> binary crate -> execution start


// if src have lib.rs -> root lib crate
fn main() {
    println!("Hello, world!");
}
