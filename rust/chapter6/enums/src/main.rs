#[derive(Debug)]
enum Message{
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        println!("{:?}", self);
    }
}

fn main() {
    let m = Message::Write(String::from("ok!"));
    m.call();
    println!("{:?}", m);

    let y : Option<i8> = Some(5);
}
