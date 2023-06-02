#[derive(Debug)]
struct Whatever {
    first: i32,
    middle: (i32, i32, i32, i32),
    last: Option<String>,
}

fn main() {
    let mut x = Whatever { first: 11, middle: (1, 2, 3, 4), last: None };
    let cond = false;

    match x {
        Whatever { first: 1 | 2, .. }  => println!("one or two"),
        Whatever { first: 3, middle: (first, .., last), .. } => println!("first: {}, last: {}", first, last),
        Whatever { first: 4 ..= 6, .. } => println!("four through six"),
        Whatever { .. } if cond => println!("if cond true"),
        Whatever { first: num_var @ 7 ..= 10, .. } => println!("seven through ten, {}", num_var),
        Whatever { first: _, middle: _, last: Some(ref y) } => println!("x.last: {}", y),
        Whatever { first: _, middle: _, last: ref mut y } => *y = Some("ok".to_string())
    }

    println!("{:?}", x);
}
