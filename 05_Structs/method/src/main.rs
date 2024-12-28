// Method Syntax
// Methods are simmilar to functions 
// declare -> fn keyword with and return some  value
// functions which are define in the context of a struct is called fuction
// define inside the struct 
// Parameter is always self intance 

// #[Derive(Debug)]
#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32
}

// implementation
// Everything within impl block will be associated with the rectangle block>
// we can pass extra arugument


//******************************
// */

// Associated Functions -> all functions define within an impl block are called assoicated functions which &self as first paramereter
// if self is not first parmeter then it is not a asscoicated fuctions -> String::from 

// generaly made to make new instance of it

// if first parameter is not self we have to call it using ::
impl Rectangle {
    // self is just like this

    // self -> self:&Self
    fn calc_area(&self) ->u32{
        self.height*self.width
    }
    // Ownership rull is still valid here
    fn parameter(&self, e:u32) -> u32{
        e*(self.height + self.width)
    }

    fn width(&self) -> bool {
        self.width> 0
    }


    fn can_hold(&self, other: &Rectangle){
        if self.height > other.height && self.width > other.width {
            println!("{self:#?} can hold {other:#?} ");
        }
        else if self.width > other.height && self.height > other.width {
            println!("{self:#?} can hold {other:#?} but in opposite sense");
        }
        else{
            println!("Not possible ");
        }
    }

    fn square (side: u32) -> Self{
        Rectangle{
            width:side,
            height:side,
        }
    }

    fn hio() {
        println!("This is new instace of it ");
    }
}

// ****************************************************
// we can choose to give a method the same name as one of the struct's fields
//******************************** */



// When we give a method the same name as a field we want it to only return the value in the field and do nothing else,
// Methods like this are called getters,

// Getters -> field private and method public
fn main() {
    // println!("Hello, world!");
    let rect = Rectangle{
        width:32,
        height:50,
    };

    // rect::hio();

    let rect2 = Rectangle{
        width:24,
        height:32,
    };

    // let area = calculate_area(&rect);
    // Method
    let area = rect.calc_area();
    let area2 = rect2.calc_area();

    let per = rect2.parameter(2);

    if rect.width(){
        println!("The rectangle has a nonzero width: it is {}", rect.width);
    }

    rect.can_hold(&rect2);



    println!("The area of rectangle {rect:#?} is {area}");
    println!("The area of rectangle 2  {rect2:#?} is {area2}");
    println!("The aparameter of rectangle 2  {rect2:#?} is {per}");


    // *******************************************************
    // *****************************************************
    let sq = Rectangle::square(5);
    println!("Sq is :{:?}", sq);


    // rect::hi();
    // we cannot implant it on any stuct  variable
    //we can only on the struct type
    Rectangle::hio();

}

fn calculate_area ( rect: &Rectangle) -> u32 {
    rect.width*rect.height
}
