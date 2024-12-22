use::std::io;

fn main() {


    println!("Fahrenheit and Celsius converter: \n");

    'main_loop: loop{

    
        println!("Enter 1: Fahrenheit to Celsius ");
        println!("Enter 2: Celsius to Fahrenheit \n");


        let so : u8;
        'sellecting_conveter: loop{
            let mut s = String::new();
            io::stdin().read_line(&mut s).expect("Unable to read!");
            let s : u8 = match s.trim().parse(){
                Ok(  num) => num,
                Err(_) =>{
                    println!("Invalid Input! . Try again");
                    continue;
                },
            };
            if s == 1 || s==2 {
                so= s;
                break 'sellecting_conveter;
            }else{
                println!("Enter a valid no 1 or 2 only");
            }

        }

        if so == 1 {
            println!("Enter temp in Fahrenheit : ");
            let mut F = String::new();
            io::stdin().read_line(&mut F).expect("Unable to read!");
            let F: f32= F.trim().parse().expect("Unable to convert");
            fahrenheit_to_celsius(F);
        }else{
            println!("Enter temp in Celsius : ");
            let mut C = String::new();
            io::stdin().read_line(&mut C).expect("Unable to read!");
            let C : f32= C.trim().parse().expect("Unable to convert");
            celsius_to_fahrenheit(C);
        }

        println!("To END  enter 0:");
        println!("To continue   enter 1:");
        
        'ending_seclector : loop {

            
            let mut con = String::new();
            io::stdin().read_line(&mut con).expect("Unable to read!");
            let con : u8 = match con.trim().parse(){
                Ok(  num) => num,
                Err(_) =>{
                    println!("Invalid Input! . Try again");
                    continue;
                },
            };
            if con == 0 {
                break 'main_loop;
            } else if con == 1 {
                break 'ending_seclector;
            }
        }

    }


}

fn fahrenheit_to_celsius (f :f32)  {
    let mut c  = f - 32.0;
    c = c*5.0/9.0;
    println!("{f} F is {c} C");
}

fn celsius_to_fahrenheit (c :f32)  {
    let mut f = c*9.0/5.0;
    f = f + 32.0;
    println!("{c} C is {f} F");
}
