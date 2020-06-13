fn main() {
    let x = "ok".to_string();
    let ok = || { // FnOnce
       x + &"ie".to_string()
    };
    println!("{:?}", ok());
    // println!("{:?}", ok()); ERROR!

    let x1 = 1;
    let ok1 = || &x1 + 1; // Fn
    println!("{:?}" , ok1());
    println!("{:?}" , x1);

    let mut x2 = 1;
    let mut ok2 = || { // FnMut
        x2 += 1;
        x2
    };
    println!("{:?}", ok2());
    println!("{:?}", x2);

    let mut x3 = vec![1, 2, 3];
    let mut ok3 = move || {
        x3.pop()
    };
    println!("{:?}", ok3());    

}