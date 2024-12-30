// --> cargo new onlylib --lib 
// then we have src/lib.rs instead to src/


// use auth_service::authenticate;
// use auth_service::Credentials;

// use auth_service::{authenticate, Credentials};

use auth_service::auth_utils::models::Credentials;
use auth_service::authenticate;

fn main() {
    // println!("Hello, world!");

    let cred = Credentials{
        username:String::from("theadityakumar"),
        password:String::from("admin123"),
    };

    authenticate(cred);
}

// Modules Cheat Sheet


// Start from the crate root: -> the compiler ->  src/main.rs(binary)  or src/lib.rs(library) 


// Declaring modules -> 
// inline within curly brackets that replace the semicolon following mod garden
// if not src/garden.rs
// then src/garden/mod.rs


// Declaring submodules
// inside garden -> mod vegetables 
// -> src/grarden/vegetables.rs
// -> src/garden/vegetables/mod.rs


