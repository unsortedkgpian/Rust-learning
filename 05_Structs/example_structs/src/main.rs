// Debug value 
// rust do not know how to print struct 
// to overcome {:?} -> is used known as debug value
// {:#?} -> pretiee debug


struct Dimensions(u32, u32);
// It helo us to use {:?} debug value to display on ternminal
#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32
}

// we can also use dbg! -> but it take ownership

fn main() {
    println!("Hello, world!");
    let w = 100;
    let h= 200;
    let rectangle_dimensions = Dimensions(w, h);

    let rect = Rectangle{
        width:40,
        height:30,
    };
    // There is no relatition between height and weight
    let area = calculate_area(w,h);
    let area = calculate_area_tuple((w,h));
    let area = calculate_area_tuple_struct(rectangle_dimensions);
    let area = calculate_area_struct (&rect);
    println!("The area of Rectangle {rect:?} is {area}", );
    // dbg!(rect); // Error take its ownership // It also tell about line no
    dbg!(&rect);
    // dbg also return the ownership
    println!("The area of Rectangle {rect:#?} is {area}", );
}
// the area function is supposed to caluculate the area of one rectangel but the fuction we wrote has two parameters and its not clear anywhere in our program that the parameters are related.
fn calculate_area(width:u32, height:u32) ->u32 {
    width*height
}

fn calculate_area_tuple(dimensions:(u32, u32)) ->u32 {
    let(width, height) = dimensions;
    width*height
}
fn calculate_area_tuple_struct(dimensions: Dimensions) ->u32 {
    let width = dimensions.0;
    let height = dimensions.1;
    width*height
}
// owenership is move so 
fn calculate_area_struct(rect:&Rectangle) -> u32{
    rect.width*rect.height
}


