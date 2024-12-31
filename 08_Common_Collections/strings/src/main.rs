fn main() {
    let hello = String::from("السلام عليكم");
    println!("hello ={hello}");
    let hello = String::from("Dobrý den");
    println!("hello ={hello}");
    let hello = String::from("Hello");
    println!("hello ={hello}");
    let hello = String::from("שלום");
    println!("hello ={hello}");
    let hello = String::from("नमस्ते");
    println!("hello ={hello}");
    let hello = String::from("こんにちは");
    println!("hello ={hello}");
    let hello = String::from("안녕하세요");
    println!("hello ={hello}");
    let hello = String::from("你好");
    println!("hello ={hello}");
    let hello = String::from("Olá");
    println!("hello ={hello}");
    let hello = String::from("Здравствуйте");
    println!("hello ={hello}");
    let hello = String::from("Hola");
    println!("hello ={hello}");



    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // let s4 = &s3 + &s2;// Error
 //********** we can only add a &str to a String; we can’t add two String values together */


 // format!

 // let s = format!("{s1}-{s2}-{s3}");

 // Bytes and Scalar Values and Grapheme Clusters 


 // unicode segemntataion crate
}
