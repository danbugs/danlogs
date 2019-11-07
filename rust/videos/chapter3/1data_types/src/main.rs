#![allow(unused_variables)]
#![allow(unused_mut)]
fn main() {
    // === scalar types
    // == integer types
    /*
    refer to: 
    https://doc.rust-lang.org/book/ch03-02-data-types.html
    for a table of scalar variables
    */
    let mut int : u8 = 255; 
    println!("u8: {}", int);

    let mut test = b'A' + 1;
    println!("test:{}",test);

    //***int = int + 1; -> this would not be caught in release!
    //***int = 256 -> this would be caught in release!
    //***println!("{}", int);

    // = in `cargo run`
    /* 
    thread 'main' panicked at 
    'attempt to add with overflow'
    */

    // = in `cargo build --release`
    // compiles just fine (wraps running (i.e. 256 -> 0))

    // == numeric operations.

    // == floating
    let float : f32 = 2.0; // f64 by default.

    // == boolean
    let boolean : bool = false;
    // variables don't have default values. 
    // must initialize them to a value.

    // == char 
    // 4 bytes (in relation to 1 of ASCII)

    let heart_eyed_cat = '😻';
    // allows us to do this ^
    println!("cat: {}", heart_eyed_cat);

    // === compound types

    // == tuple

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    let mut tup2 = (1, 2, 3, 4);
    println!("The value of y is: {}", tup.1);
    println!("The value of tup0 is: {}", tup2.0);
    tup2.0 = 0b011;
    println!("The value of tup0 binary is: {:b}", tup2.0);
    let mut tup3 = (tup2, 5);
    println!("The value of tup4 is: {}", (tup3.0).1);

    // == arrays
    // tuples where every element has the same type
    // can't change size (use vectors)

    let months = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "November", "December"];

    let a : [i32; 5] = [1, 2, 3, 4, 5];
    println!("a[0]:{}", a[0]);

    // if you want to instantiate an array with n repeated elements:

    let a = [3;5];
    // a = {3, 3, 3, 3, 3};

    // if you try to access an element outside of an array, it panics
    // not true in other low-level code. We will access wrong memory addresses.
}
