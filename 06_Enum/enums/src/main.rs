// Enumerations -> enums
// Struct ->  give you a way of grouping together related fields and date
// enums -> give you a way of saying a value is one of a possible set of values



// Example IP Address
// Two standers -> ipv6 and ipv4

// # [derive(Debug)]
// enum IpAddrKind{
//     V4,
//     V6,
// }


// We can also use derive Debug string
# [derive(Debug)]
struct IpV4Addr(u8, u8, u8, u8);

# [derive(Debug)]
struct IpV6Addr(String);

# [derive(Debug)]
enum IpAddrKind{
    // V4(String),// In case of struct the variantent type must me same but not in this 
    // 
    // V4(u8, u8, u8, u8),
    V4(IpV4Addr),
    V6(IpV6Addr),

    // V6(String),
}

// struct IpAddress {
//     address:String,
//     kind:IpAddrKind,
// }

// impl IpAddress {
//     fn new(address:String) -> Self{
//         Self{
//             address,
//             kind:IpAddrKind::V4
//         }
//     }
// }

fn main() {
    println!("Hello, world!");

    // route("1.2.3.4", IpAddrKind::V4);

    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let ip = IpAddress{
    //     address:String::from("1.2.3.4"),
    //     kind:IpAddrKind::V4
    // };

    // let google_ip = IpAddress::new(String::from("1.2.3.54"));

    // route(&google_ip);

    // // route("2.34.32.32", four);
    // route(&ip);

    // let home = IpAddrKind::V4(127, 0 , 0, 1);
    // let ip = IpAddrKind::V6(String::from("127.00.00.00.01"));


    let uou4 = IpV4Addr(23, 0, 89, 1);
    let uou6 = IpV6Addr(String::from("234.00.23.45.64"));

    // let home = IpAddrKind::V4(127, 0 , 0, 1);
    // let ip = IpAddrKind::V6(String::from("127.00.00.00.01"));
    let home = IpAddrKind::V4(uou4);
    let ip = IpAddrKind::V6(uou6);
    route(&ip);// Takes the ownerhsip so 
    route(&home);

    println!("{:?}",ip);

}



// fn route(ip:&str , kind:IpAddrKind) {
//     println!("Routing request to IP {ip} of kind {kind:?}");
// }

// fn route(ip: &IpAddress) {
//     // println!("The ip address is {ip.address}");// Error
//     // rust donot allows directyle access the field in {} so 
//     println!("The ip address is {} of kind {:#?}", ip.address, ip.kind);
// }
fn route(ip: &IpAddrKind) {
    // println!("The ip address is {ip.address}");// Error
    // rust donot allows directyle access the field in {} so 
    println!("routing {:#?}", ip);
}