fn main() {
    let s1 = String::from("hello"); // s1 comes into scope
    
    let s2 = s1; // s2 comes into scope

    // s1 got moved onto s2
    // OR s1 transfered ownership of it's binded value to s2.
    
    println!("{}", s1); // can't do this
} 

// double free eror (?) ... Ownership!
