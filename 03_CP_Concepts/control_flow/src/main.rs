fn main() {
    // println!("Hello, world!");//

    //If and loops
    // if want to run a code on basis of condition is called control flow

    let num = 3;
    // if expression are sometimes called arms
    // codtions should always be boolen
    // other data types are not allowed unlike other programing langauge
    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //     let x = 5;
    //     if x{
    //         println!("number is valid");
    //     }// Throw a error

    // Handling multiple conditions
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 2, or 2");
    }

    // if is an expression

    let condition = true;
    let number = if condition {5} else {6};
    // if and else should have same data types
    // let p = if condition {7} else {"helro"}// error
    println!("The value of number is :{number}");

    // **********************************************************
    // *****************************Loops*************

    // 3 types of loop
    // 1.loop 
    // 2.while
    // 3.for


// loop is an expression



    // loop -. forever loop until you tell to stop
    // execute a block of code over and over again forever or until you explicitly tell it to stop
    


    // Infinit loop

    // let mut po = 1;
    // loop{
    //     println!("{po}");
    //     po= po+1;
    // }

    // let mut po = 1;
    // loop{
    //     println!("{po}");
    //     po = po +1;
    //     if po == 10{
    //         break;
    //     }
    // }

    // Trick question

    // overflow case

    // let mut pk:u8 = 1;
    // loop{
    //     println!("Value of num is {pk}");

    //     if pk ==10 || pk == 0{
    //         break;
    //     }
    //      pk = pk +pk;

    // }





    // Returning values from loops


    // loop is expression we can return value;
    // let mut num = 1;
    // let result = loop{
    //     println!("Value of number is {num}");
    //     if num == 11{
    //         break 70;
    //     }
    //     num = num +1;

    // };

    // println!("The result is {result}");





    // loop labels in Rust in nested loop 
    // it should start from ' in strarting point

    'main_loop: loop{
        println!("This is fun");
        let mut n=10;
        'second_loop: loop{

            println!("The value of n is {n}");
            if n == 20{
                // println!("The value of n is {n}");
                break 'main_loop;

            }
             n = n+1;

        }

    }


    // ***************************************

    // While loop

    let mut po = 1;

    while po<10{
        println!("The value of po is {po}");
        po = po + 1;
    } 

    let arr = [1,2, 3, 4, 5 ,6 ];
    let mut index = 0;

    while index < 6 {
        println!("i: {} and v: {}", index, arr[index]);
        // It is sole every time i call arr[index] compiler adds runtime code to perfome the conditional check of whether the index is within the bounds of the arry of not
        index +=1;
    }
    // This approch is error prone




    // For Loop


    let ary = [34, 22, 643, 212, 225, 233];

    for elem in ary {
        println!("the value is {elem}");
    }
    

    for x in 1..=100{
        println!("x is {x}");
    }
    for x in (1..=100).rev(){
        println!("x is {x}");
    }

}
