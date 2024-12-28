struct User {
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}

// tuple structs 
struct Color(u8, u8, u8);

struct Point(u8, u8, u8);

struct AlwasyEqual;// Unit-like Struct

fn main() {
    // println!("Hello, world!");

    //Instanceof 
    let user_1= User {
        email:String::from("theadityakumar2810@gmail.com"),
        username:String::from("AdityaK2810"),
        active:true,
        sign_in_count:0,
    };

    
    let mut user_2 = User {
        email:String::from("theadityakumar2811@gmail.com"),
        username:String::from("A2810"),
        active:true,
        sign_in_count:0,
    };

    println!("The name of user is {}", user_1.username);

    // user_1.username = String::from("Aditya Kumar");// Immutable

    user_2.username = String::from("AK2810");
    user_2.username.push_str("AKjdfhiujah2810");

    
    println!("The new username of user 2 is {}", user_2.username);
    
    // all ownership rule will be followed
    
    let s1= user_2.username;// Ownership is moved to s1
    //we can assign the value to user1 username
    
    user_2.username= String::from("the new value ");// 
    println!("The username of user 1 is {}", user_2.username);//

    // If mutable the whole instace is mutable 
    // if some part cant be immutable and mutable at same point

    let user_3 = build_user(String::from("Aitydtyei"), String::from("the@gmail.com"));

    println!("The user 3 username is {}", user_3.username);





    // tuple vs Struct
    let co:(u8, u8, u8) = (10, 20, 40);// tuple
    let po:(u8, u8, u8) = (10, 34, 33);// tuple\

    let new_color = Color(100, 0, 0);
    let new_point = Point(30, 40, 9);
 

    set_bg_color(po);
    move_point(co);
    // set_bg_color_safe(po);// Error
    // move_point_safe(co); // Error

    set_bg_color_safe(new_color);
    move_point_safe(new_point);


    // ***************************

    // Unit-Like structs without any fields
    // we can use struct tuple to safe gurd type of inputh when we dont have data 


    let subject = AlwaysEqual;
}

// Return ownership of new user


fn build_user(username:String, email:String) -> User {
    User {
        // username:username,
        // email:email,
        // for convenient we can user shorthand
        username,
        email,
        active:true,
        sign_in_count:0,

    }

    // Tuple Vs Structs
    // tuple structs -> rust also supports structs that look similar to tuples

}

// any tuple can access this funtion 
// so we can create a tuple for type safetya
fn set_bg_color(color:(u8, u8, u8)){
    println!("Setting Background Color R={}, G={}, B={}", color.0, color.1, color.2);
}

fn set_bg_color_safe(color:Color){
    println!("Setting Background Color R={}, G={}, B={}", color.0, color.1, color.2);
}

fn move_point(point:(u8, u8, u8)){
    println!("Shifting position to  X={}, Y={}, Z={}", point.0, point.1, point.2);
}
fn move_point_safe(point:Point){
    println!("Shifting position to  X={}, Y={}, Z={}", point.0, point.1, point.2);
}
