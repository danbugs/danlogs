use std::fs::{File};
use std::io::{self, ErrorKind, Read};

fn main() {
    // # ERRORS
    // - Two types:
    //  - Unrecoverable errors, and
    //  - Recoverable errors.

    // ## Unrecoverable Errors.
    // let v = vec![1,2,3];
    // v[99]; // panic!()
    // panic!("crash!!!");

    // ## Recoverable Errors.
    // enum Result<T,E>{
    //     Ok(T),
    //     Err(E),
    // }

    // - Common programming pattern & Matching other errors
    // let f = match File::open("hello.txt"){
    //     Ok(file) => file,
    //     Err(e) => match e.kind(){
    //         ErrorKind::NotFound => match File::create("hello.txt"){
    //             Ok(file2) => file2,
    //             Err(e2) => panic!("{:?}", e2),
    //         },
    //         _ => panic!("no clue what happened"),
    //     },
    // };

    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("oof");

}

fn read_username() -> Result<String, io::Error>{
    // let mut f = match File::open("username.txt"){
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s){
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // let mut f = File::open("username.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
