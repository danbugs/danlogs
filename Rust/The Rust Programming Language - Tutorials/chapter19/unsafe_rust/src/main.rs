static _HELLO_WORLD: &str = "Hello, world!";
const _HELLO_WORLD2: &str = "Hello, world!";

static mut COUNTER: i32 = 0;

fn main() {
    unsafe { COUNTER += 1; }

    println!("COUNTER: {}", unsafe { COUNTER });
}






















// fn main() {
//     let mut num = 5;

//     let r1 = &num as *const i32;
//     let r2 = &mut num as *mut i32;

//     println!("r1 is: {:#?}", r1);
//     println!("r2 is: {:#?}", r2);

//     let deref_r1 = unsafe { *r1 };
// }
