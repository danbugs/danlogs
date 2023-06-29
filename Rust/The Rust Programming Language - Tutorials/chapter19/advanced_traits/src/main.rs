trait A<T = i32> {
    fn method(_: T);
}

struct MyStruct;

impl A for MyStruct {
    fn method(_: i32) {
        println!("This is A.");
    }
}

impl A<f64> for MyStruct {
    fn method(_: f64) {
        println!("This is A<f64>.");
    }
}

fn main() {
    <MyStruct as A>::method(1);
    <MyStruct as A<f64>>::method(1.0);
}







































// trait A {
//     fn method();
// }

// trait B {
//     fn method();
// }

// struct MyStruct;

// impl A for MyStruct {
//     fn method() {
//         println!("This is A.");
//     }
// }

// impl B for MyStruct {
//     fn method() {
//         println!("This is B.");
//     }
// }

// fn main() {
//     <MyStruct as A>::method();
//     <MyStruct as B>::method();
// }
