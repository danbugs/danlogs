const DAN: i32 = 241;

struct Coordinate<'a, 'b>{
    x: &'a i32,
    y: &'b i32,
}

fn x_or_const<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32{
    if x > y{
        x
    }else{
        &DAN
    }
}

fn main(){
    let x = 5; // LP1
    let r; 
    {
        let y = 10; // LP2
        r = x_or_const(&x, &y);
    }// /LP2
    println!("{}", r);
} // /LP1