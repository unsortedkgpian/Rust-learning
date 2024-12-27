use std::io;

fn main() {
    println!("Hello, world! ");
    println!("Fibonacci number Generator!");
    println!("Enter a nth number");

    let  nth :u32 ;
    loop{
        let mut ntho = String::new();
        io::stdin().read_line(&mut ntho).expect("Failed to read line");
        
        let ntho: u32 = match ntho.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input try again");
                continue;
            }
        };
        nth=ntho;
        break;
    }

    let f:u128=fibo(nth);

    println!("The {}th Fibonacci number is : {}", nth, f)

}

fn fibo(nth:u32) -> u128 {
    if nth == 0 {
        return 0;
    } else if nth ==1 {
        return 1;
    }

    fibo(nth-1) + fibo(nth-2)
}
