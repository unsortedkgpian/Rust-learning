fn main() {
    println!("Hello, world!");

    let mut s1 = String::from("Aditya Kumar");

    // & is called refrence with taking ownership
    let len = str_len(&s1);

    println!("The length of string: {} is : {}", s1, len);


    // The oppoiste of referencing & is dereferincing * 
    // this is call borrowing



    // Mutable reffrence 

    let len = mut_ref(&mut s1);

    println!("The length of string: {} is : {}", s1, len);


    //**************************************************************************************
    // */

    // Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value 

    let r1 = &mut s1;
    // It will give error becuse we have already given a mutabele refrence 

    // let len = mut_ref(&mut s1);
    println!("{}", r1);
    // Both give error
    // let r2 = &mut s1;
    // let r3 = &s1;

    // println!("{}, {}", r1, r2);// Error
    // if will break the security of the code



    // To prevent data races at comile time
    // 1.Two or more pointers access the same data at the same time.
    // 2. At least one of the pointers is being used to write to the data.
    // 3. there's no mechanism beign used to synchronize acess to the data.

    // The refrence is valid until the last use
    // Then  we can use the muttable refrence in use




    // ****************************************
    // Dangling References
    // Dangling Pointer -> a pointer that references a location in memory that may have been given to someeone else-by freeing some memory while preserving a pointen to that memory



    let pi = dangle();



}


// this s is pointer where the stack memory of s1 where its pointer is store
fn str_len(s:&String) -> usize{
    // s.push_str("Hello world");// error 
    // it is immutabe because we are taking the refrence


    // By default all refrence are immutable
    s.len()
}

fn mut_ref (s: &mut String) ->usize{
    s.push_str("Muatable refrence");
    s.len()
}


fn dangle() -> String{
    let s = String::from("Gikiu");
    // &s // Error dangleing refrence its owner died when funtion is over so not valid
    s
    
}
// fn dangle() -> &String{
//     let s = String::from("Gikiu");
//     // &s // Error dangleing refrence its owner died when funtion is over so not valid

// }
