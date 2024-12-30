pub mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {
            // Relative path
            super::hosting::seat_at_table();
        }

        fn take_payment() {}
    }
}

// Paths for referring to an item in the module tree
// two types path 
// Absolute path -> start with crate
// Relative path -> start with self , super or an identifier in the current module

// seperated by ::

pub fn eat_at_restaurant(){
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path 
    // same level of the module
    front_of_house::hosting::add_to_waitlist(); // it can acces its relative funtion without pub keyword -> if same level 
}

// If we make struct all fields will be private we have make them public one by one
// But none in case of enum all flow the main enum


/// use only work in current scope not work in nested scope
/// 
/// // using as keyowrkd we can relname
/// 
/// 
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}