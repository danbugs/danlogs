// use std::ops::Deref;
// use std::ops::DerefMut;

// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &T {
//         &self.0
//     }
//     // *y -> *(y.deref())
// }

// impl<T> DerefMut for MyBox<T> {
//     fn deref_mut(&mut self) -> &mut T {
//         &mut self.0
//     }
// }

// fn main() {
//     let x = 5;
//     let mut y = MyBox::new(x);
//     *y = 10;
//     assert_eq!(5, *y);
// }










// struct Pair(i32, i32);

// trait Coordinate {
//     type A;
//     type B;

//     fn x(&self) -> i32;
//     fn y(&self) -> i32;
// }

// impl Coordinate for Pair {
//     type A = i32;
//     type B = i32;

//     fn x(&self) -> i32 { self.0 }
//     fn y(&self) -> i32 { self.1 }
// }

// fn y_diff<C: Coordinate>(coor_1: &C, coor_2: &C) -> i32 {
//     coor_2.y() - coor_1.y()
// }

// fn main() {
//     let coor_1 = Pair(1, 2);
//     let coor_2 = Pair(3, 4);

//     dbg!(y_diff(&coor_1, &coor_2));
// }








// fn hello(name: &str) {
//     println!("Hello, {}!", name);
// }

// fn main() {
//     let m = MyBox::new(String::from("Rust"));
//     hello(&m); // instead of &(*m)[..]
// }








struct Whatever(i32);

impl Drop for Whatever {
    fn drop(&mut self) {
        println!("Dropping Whatever with data: {}!", self.0);
    }
}

fn main() {
    let _a = Whatever(1);
    std::mem::drop(_a);
    let _b = Whatever(2);
}