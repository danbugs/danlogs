const DAN : i32 = 241;

struct Coordinate<'a, 'b>{
    x: &'a i32,
    y: &'b i32
}

fn x_or_const<'a>(x: &'a i32, y: &'_ i32) -> &'a i32{
    if x > y{
        x
    }
    else{
        &DAN
    }
}

fn main() {
    let x = 5;
    let r;
    {
        let y = 10;
        let c = Coordinate{x: &x, y: &y};
        r = x_or_const(c.x, c.y);
    }
    println!("{}", r);
}