fn main() {
    // println!("Hello, world!");

    // Data type in rust is static you cannot change it directly unlike JS
    let mut a: i32 = 1;
    let b;
    b = 10;
    // Scalar Types and Dyanmic Types
    //

    // ********************************************************
    // Scalar type
    // 1.Intergers
    // 2.floating-point
    // 3.Booleans
    // 4.characters
    // *********************

    // Intergers
    // two notations i and u
    // u-> unsiged interger
    // u:8      i:8
    // u:16     i:16
    // u:32     i:32
    // u:64     i:64
    // u:128    i:128
    // arch usize and isize => auto detact the system size
    // two's compelement store the sign (+,1 in terms of 0 and 1) in most significant bit in database
    // in  can store -(2^(n-1)) to (2^(n-1) -1 )
    // un can stroe 0 to (2^n  - 1)
    // funtion auto detact th
    let c = 23u8;
    // we can also decalare funtion like this
    //**********  ****************** ***************************/
    // Visiwal seperator very important
    let s = 1000000000; //very diffuct to read
    let s = 1_10_00_000;

    let t = 327823_i32;
    // Decimal
    // Hex  -> 0xff
    // Octal -> 0o77
    // Binary -> 0b11111_000
    // Byte => b'A'

    let u = 0b111_00_101_110;
    println!("The value of u is {u}");

    let e = b'c';
    println!("The Value of e is {e}");

    // Default type of interger is i32;
    // debug includes check interger overflow
    // debug mode default mode it check overflow pacinking program exxit
    // ***********
    // But when compilling in release mdoe  with --release  flag rust not include check
    // If overflow occur it from two's complement wrapping
    // it it overflow it start from min and continue unstill all is done
    // let t:u8 = 289;

    let mut t: u8 = random_number() + 10;

    println!("the value of t is {t}");
    // In cargo run is paniced
    // In cargo run --release it use tow's complement wrapping

    // To save it from overflow we should use wrapping_* mettods

    let t: u8 = random_number().wrapping_add(20);
    println!("the value of t is {t}");

    /* 1. wrapping_* Methods
    These methods wrap around on overflow.

    wrapping_add
    wrapping_sub
    wrapping_mul
    wrapping_div (Division by zero still panics.)
    wrapping_rem
    wrapping_neg (Signed types only; wraps around on overflow.)
    wrapping_shl (Left shift that wraps on overflow.)
    wrapping_shr (Right shift that wraps on overflow.)
    wrapping_abs (Signed types only; wraps around on overflow.)
    wrapping_pow


    2. checked_* Methods
    These methods return None on overflow, preventing runtime errors.

    checked_add
    checked_sub
    checked_mul
    checked_div (Returns None on division by zero.)
    checked_rem (Returns None if divisor is zero.)
    checked_neg (Signed types only; returns None on overflow.)
    checked_shl (Returns None if the shift amount is invalid.)
    checked_shr (Returns None if the shift amount is invalid.)
    checked_abs (Signed types only; returns None on overflow.)
    checked_pow
    3. saturating_* Methods
    These methods clamp the result to the minimum or maximum value on overflow.

    saturating_add
    saturating_sub
    saturating_mul
    saturating_neg (Signed types only; clamps to the maximum positive value.)
    saturating_abs (Signed types only.)
    saturating_pow
    4. overflowing_* Methods
    These methods return a tuple: (result, bool) where the boolean indicates if an overflow occurred.

    overflowing_add
    overflowing_sub
    overflowing_mul
    overflowing_div (Division by zero still panics.)
    overflowing_rem
    overflowing_neg (Signed types only; wraps around on overflow.)
    overflowing_shl (Left shift; indicates if bits are shifted out.)
    overflowing_shr (Right shift; indicates if bits are shifted out.)
    overflowing_abs (Signed types only; indicates overflow.)
    overflowing_pow
    */

    // let y:u8 = match random_number().checked_add(145) {
    //     Some(num) => num,
    //     None => {
    //         println!("Cannot add");
    //         return;
    //     }
    // };
    // println!("The value of y is {y}");

    // let op:u8 = match random_number().checked_add(145) {
    //     Some(num) => num,
    //     None => {
    //         println!("Cannot add");
    //         return;
    //     }
    // };
    // println!("The value of op is {op}");

    let (bg, boo): (u8, bool) = random_number().overflowing_add(150);
    println!("The value of bg is {} and boo is {}", bg, boo);

    // **************************************************************************
    // Floating-Point
    // only two types f64 and f32

    let pu = 3.8;
    let po: f32 = 5.7;
    // f64 is default type of floating point number
    // f64 is double prission compare to f32
    let ui = 1.123456789012345678901234567890;
    let uo: f32 = 1.123456789012345678901234567890;
    println!("ui of f64 is {ui}");
    println!("uo of f32 is {uo}");

    //***********************************************************
    // *************************************************** */
    // Numberic Operation
    // Same to C and C++ programing
    // mention the float during division is important

    /// Boolean type(bool)
    /// true or false

    /// Charater type
    let iot = 'c';
    let iot: char = 'Z';
    let bh = "o"; // its a string

    // char have 4 bytes storage
    // utf encoder
    // we can also stroe emojie

    //********************************************************************* */
    // *********************************************************************

    // The Compound Datatypes
    // 1. Tuple
    // 2. Array

    // Tuple is grouping with varites of datatypes
    // let tup:(i32, u8, String ) = (4, 9, "aditya");// It is showing erro "aditya" has type &str

    let tup: (i32, u8, String) = (4, 9, String::from("aditya"));
    println!("The tup is {}", tup.2);

    // Ones declare we cant grow or srink it
    // we can desturce a tup like object in JS
    let (one, two, three) = tup;
    println!("The value of one is {}", one);
    println!("The value of two is {}", two);
    println!("The value of three is {}", three);

    // there also a tuple where there in ot value in it

    let tup = (); // It is called Unit
                  // helpull in stroring the result

    // Array

    let bc = [1, 3, 6, 9];
    let pk: [u8; 3] = [1, 2, 3];
    // Array is not flexible
    // Vector type is flexible

    let dc = [10; 5];
    // println!("Value of dc[6] is {}",  dc[6]);
    // Erro out of bounds

    let index = random();
    println!("Value is {}", dc[index]);
}

fn random_number() -> u8 {
    200
}

fn random() -> usize {
    7
}
