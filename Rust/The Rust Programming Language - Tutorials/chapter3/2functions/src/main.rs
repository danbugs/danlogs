fn main() {
    let mut x = 10; // statement (i.e. doesn't return a value)
    let mut x = stuff(x);
    println!("after stuff(): {:?}", x);
    fn one() -> u32{
        1
    }
    let mut o = one();
    println!("o1 = {}", o);
    let y = {
        let a = 3;
        println!("inside y: a = {}", a);
        a + 1 // expression (i.e. returns a value)
    };
    println!("y = {}", y)
    //println!("{}", a); -> outside of scope

    // expressions don't end with ';'
}

fn stuff(mut x: i32) -> i32{ // must declare type
    x = 2;
    println!("inside stuff(): {}", x);
    x
}