enum Message {
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32)
}

// Option Enum -> prebuilt

enum Option(u:32) {
    None,
    Some(true)
}

fn main() {
    // println!("Hello, world!");

    let ar = Option<1>;

    // is_some
    // unwrap

    let b = 1;
    
    // Error
    // let c = b+ar;

    // You can't add int type and options{int}
    // You have to convert it first

    let c = b + ar.unwrap();
    // if value mismatch or None code will pankic

    //unwrap_or(0)

    let d = b + ar.unwrap_or(b);

}
