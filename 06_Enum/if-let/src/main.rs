fn main() {
    // println!("Hello, world!");

    let config_max = Some(3u8);
    // let config_max :Option<u8> = None;
    // here we are addressing only Some but we have to none arm -> if let solution

    // match config_max {
    //     Some(max) => println!("the maximum is configured to be {max}"),
    //     _ => (),
    // }

    if let Some(max) = config_max{
        println!("The maximum is configured to be {max}");
    } 
}
