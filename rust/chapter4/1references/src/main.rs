fn main() {
    let mut s1 = String::from("ok1");
    // Rule #1
    // let r1 = &mut s1;
    // let r2 = &mut s1;

    // println!("{} {}", r1, r2);
 
    // Rule #2
    let r1 = &s1;
    let r2 = &mut s1;
    println!("{} {}", r1, r2);

    // Dangling References
    let r = dangle();
}

fn dangle() -> &String{
    let s = String::from("ok");
    &s
}
