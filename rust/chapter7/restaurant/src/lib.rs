mod front_of_house; // src/front_of_house.rs

use crate::front_of_house::hosting;

fn eat_at_restaurant(){
    hosting::add_to_waitlist();
}




















// use std::cmp::Ordering;
// use std::io;
// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
// use std::io::{self, Write};

// use std::collections::*;















// mod front_of_house{
//     pub mod hosting{
//         pub fn add_to_waitlist(){
//             hosting_chaos::hosting2::add_to_waitlist();
//         }
//         pub mod hosting_chaos{
//             pub use crate::front_of_house2::hosting2;
//         }
//     }
// }

// mod front_of_house2{
//     pub mod hosting2{
//         pub fn add_to_waitlist(){}
//     }
// }

// use front_of_house::hosting::add_to_waitlist;
// use front_of_house2::hosting2::add_to_waitlist as add_to_waitlist2;

// fn eat_at_restaurant(){
//     add_to_waitlist();
//     add_to_waitlist2();
// }


























// mod front_of_house{
//     fn do_something(){}
//     pub mod hosting{
//         pub fn add_to_waitlist(){
//             crate::eat_at_restaurant();
//             //crate::front_of_house::do_something();
//             super::do_something();
//         }
//     }
// }

// fn eat_at_restaurant(){
//     // Absolute Paths
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative Paths
//     front_of_house::hosting::add_to_waitlist();

//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     meal.toast = String::from("Wheat");
//     //meal.seasonal_fruint -> NOP!
// }

// mod back_of_house{
//     pub struct Breakfast{
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     pub enum Appetizer{
//         Soup,
//         Salad,
//     }

//     impl Breakfast{
//         pub fn summer(toast: &str) -> Breakfast{
//             Breakfast{
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }