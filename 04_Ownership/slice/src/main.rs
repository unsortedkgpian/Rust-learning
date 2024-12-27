use std::any::type_name;// It give the data type in return

fn main() {
    // println!("Hello, world!");
    // The SLICE TYPE
    // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection . A slice is kind of refreence, so it does not have owenship


    // a string of words seperated by spaces and return the fist word it finds in that string,

    let mut s = String::from("rust the new superstat of coding world");

    let len = first_word(&s);

    // s.clear();

    println!("The first word length of the string : {s} is : {len}", );

    // A string slice is a reference to part of a String

    let rusto = &s[0..4];
    let the = &s[5..8];

    // s.clear();// Error
    // is reference is still valid here so its showing error
    // just like you have 2 mutable reference;

    // slice type have only pointer and len don't have capacity in it


    println!("rusto is {rusto}");
    println!("the is {the}");

    let a = &s[..9];// Strarting from one
    let b = &[10..];// To the end;
    let c = &s[..];// Strarting to the end;

    s.clear();

    let to = "Hello World!";
    // It is hard process so the rust store it in form of binaay in runtime


    // **********************************************

    // the type of s is &str:: its a slice pointing to that spcecfic point of the binary
    // This is also why string literals are immutabel &str is an immutable reference


    // String Slices as Parameters


    // Knowing that you can take slices of literals and String

    // we cannot the truncate the String it needs to get a mutable references  
    // Its a slice pointing to sppecific point fo binary
    // dont have cpaicity and immutable and tyep &str (slice) immutabe referecase

    // we can take &str and &String for immutable referecne


    //*********deref coercions******* */

}


fn first_word(s:&String) -> usize {
    let bytes = s.as_bytes();// as_bytes retrun all char in array
    // dbg!(bytes); Return the value
    // println!("The data type of bytes is {}", type_name::(bytes));
    
    // let mut len =1;

    // for *bito in bytes{
    //     if bito == b' ' {
    //         break;
    //     }
    //     len+=1;
    // }
    // len

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice (sto:&String) -> &str{
    let s = sto.as_bytes();

    for(i , &item) in s.iter().enumerate(){
        if item == b' '{
            return &sto[..i];
        } 
    }

    &sto[..]
}