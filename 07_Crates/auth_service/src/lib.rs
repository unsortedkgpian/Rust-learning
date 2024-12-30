// #![allow(dead_code, unused_variables)]

// // By defalut all thing in lib is private 
// // To use it we have to use pub
// pub struct Credentials {
//     pub username:String,
//     pub password:String
// }

// enum Status{
//     Connected,
//     Interrupted,
// }

// fn connect_to_database() -> Status {
//     // logic of connection
//     Status::Connected
// }

// fn get_user() {
//     // fetch the user from the database and return
// }

// fn logic(cred:Credentials) {
//     // try tol
//     get_user()
// }

// pub fn authenticate(cred:Credentials){
//     if let Status::Connected = connect_to_database() {

//     }
// }

mod database;

pub mod auth_utils;

// pub mod auth_utils {


//     // we can acess the mod in same mod directly
//     pub fn login(cred:models::Credentials) {
//         // try to login the user
//         // get_user()

//         // crate::database::get_user()
//         super::database::get_user()
//     }

//     pub mod models{
//         pub struct Credentials {
//             pub username:String,
//             pub password:String,
//         }   
//     }
// }


pub fn authenticate(cred : auth_utils::models::Credentials){
    if let database::Status::Connected = database::connect_to_database() {
        auth_utils::login(cred);
    }
}

// we can add file
mod util;
// it will serarch -> src/utils.rs then src/util/mod.rs