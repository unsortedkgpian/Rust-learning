fn main() {
    // let mut a ;
    // a= 10;
    // a = 20;
    // let b  : i64 = 42;
    // let mut c : i64 = 1000;
    // c=98_222;
    // c=0b10101;
    // c=10_00_000_000_00;
    // let c=b'a';
    // let mut i = 0;
    // loop{
    //     i=i+1;
    //     println!("{c}");
        
    // }
    // --release flag =>code wont paniked if any thing with overflow it will auto qrap it

    // let z: i8 = random_number().wrapping_add(57);
    // let z: i8 = match random_number().checked_add(57){
    //     Some(num) => num,
    //     None =>{
    //         println!("Cannot add");
    //         return;
    //     }
    // // };
    // let (a, b)  = random_number().overflowing_add(50);

    // println!("Value of a is {a} and b is {b}");
    // let x = 2.0;
    // let y :f32 = 43.00;
    // let my_f64 = 2.123456789123456789;
    // let my_f32: f32 = 2.123456789123456789;
    // println!("my_f64 is : {my_f64}");
    // println!("my_f32 is : {my_f32}");

    // let x :f32 = 5f32/2f32;


    // println!("X is {x}");



    let tupe:(i32, &str, f32, char) = (2, "sit" , 3.0, 'f');
    println!("{:?}", tupe);
    let b:[i32; 6] = [10; 6];
    println!("{:?}", b);
}

// fn random_number() -> u8{
//     100
// }
