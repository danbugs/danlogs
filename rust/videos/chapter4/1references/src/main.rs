fn main() {
    // let r = dangle();
    let mut s = "ok";
    println!("{}", s);
    s = "ok2";
    println!("{}", s);
}

// fn dangle() -> &String{
//     let s = String::from("ok");
//     &s 
//     /*
//     returns a reference to a
//     value that is about to be dropped
//     a line earlier (i.e. reference
//     will point to something completely
//     unrelated).
//     */
// }