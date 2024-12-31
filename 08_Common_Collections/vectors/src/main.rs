// fn main() {
//     println!("Hello, world!");
// }

// Collections can contains multiple values 
// data stores on the heap -> data can grow or shrink

// Vector
// String
// hash map

#[derive(Debug)]
enum SpreadSheetCell {
    Text(String),
    Int(i32),
    Float(f64),
}

fn main(){
    let mut v:Vec<i32> = Vec::new();
    // all type should be same 
    // if we are making vec without any element we have to annontate a signle

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    let v = v;//Shawdoing 

    println!("vec = {:?}",v);


    let veco = vec![1,3,5,98,23,64];

    println!("vec = {:?}",veco);

    // let fourth_value = veco[3];
    // ownership is moved 
    // chases of crash

    // let fourth_value = veco.get(3);// it will return option
    // let fourth_value = veco.get(3).unwrap_or(&-1);

    let fourth_value = match veco.get(30){
        Some(num) => num,
        None => {
            println!("The index is out of bound");
            &-1
        }
    };

    println!("vec = {:?} and the fourth value is {fourth_value}",veco);

    // Iterate over vector

    for i in &veco{
        println!("The value of i is {i}");
    }

    // i is pointer 
    // to perfome some task on i we have to reference it

    let mut v1 = vec![1,2,4,5,6,7];
    for i in &mut v1 {
        println!("The value of i in v1 is {i}");
        *i *=2;
    }

    println!("The Vector after the loop is {:?}", v1);

    // What if we want to store a mutiple type in  a vector we need to make a enum

    let cells : Vec<SpreadSheetCell> = vec![SpreadSheetCell::Text(String::from("Aditya")), SpreadSheetCell::Int(32), SpreadSheetCell::Float(34.3223)];

    println!("The cells vector is {:?}", cells);

}