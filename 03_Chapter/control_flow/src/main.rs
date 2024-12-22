fn main() {
    let mut num = -5;
    if num < 5 {
        println!("Hello, world!");
        
    } else {
     println!("Hello, Hell")   ;
    
    }

    loop{
        println!("Value of\n number is {num}");
        
        if num>=10{
            break;
        }
        num = num +1;

    }
}
