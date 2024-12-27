fn main() {
    println!("Hello, world!");
    another_function();
    fun_function(32);
    let y = six(5);
    println!("The y is even or not : {y}");
}
// rust code use snake case -> if multiple words use undersore and lowercase
// It dont matter where you have difne the funtion up or last rust will search the main funtion


fn another_function() {
    println!("This is new functions");
}


// **********Parameters ***********

// It is imput for a fuctions

//*************Statments vs Expressions */
// Statments are instruction that perfome some task and do not return a value
// Expressions evaluate to a resultant value, Lets Look, at some examples
// Expressions can be part of a statment 


// type annotion in funtion is must
fn fun_function(x:i32){


    let y = 5 ;// Statements
    let z = 45 + 82;
    // 45+82 is expression
    // which is part of statment
    // declaring funtion is statemnt 
    // But calling a fuction is expression it return a value

    let p = pic();

    // Calling a macro is an expression
    println!("The value of input/Parameters is {x}");// expression
    // new scope block create with curly brackets is an expression
}

fn pic() {
    println!("He how are you");


    let y = {
        let x=3;
        x+1
    };

    // In scope the statement without the semiclolen is return statment

    // Expressions do not include ending semicolons
}

// Fuctions with return Values
// we can aslo use return value 
// without retrun and semicolons is favarable

fn five () -> i32 {
    // 5; it become a statement

    // Return key word is used in eary return
    5
}

fn six(x:i32) -> bool {
    if x%2 ==0 {
        // true //it will show error 
        // For early return
        // return true;
    }
    println!("Value is not even");
    false
}