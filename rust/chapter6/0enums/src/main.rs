fn main() {
    // Creating enums
    #[derive(Debug)]
    enum Coin{
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    // Accessing enums
    let c : Coin = Coin::Penny; 

    // Enums with data
    #[derive(Debug)]
    enum Coin_Data{
        Penny,
        Nickel,
        Dime,
        Quarter(String),
    }

    // impl in enums
    impl Coin_Data{
        fn call(&self){
            println!("{:?}", self);
        }
    }

    let c_d : Coin_Data = Coin_Data::Quarter(String::from("lucky quarter"));

    c_d.call();

    // Rust doesn't have null 
    // https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/

    // Option<T> enum (INCLUDED IN PRELUDE!)
    // enum Option<T>{
    //     Some(T),
    //     None,
    // }

    let o : Option<u8> = Some(5);

    // match
    match c {
        Coin::Penny => println!("{:?}", c),
        Coin::Nickel => println!("{:?}", c),
        Coin::Dime => println!("{:?}", c),
        Coin::Quarter => println!("{:?}", c),
    }

    // value binding in match
    let vc = value_in_cents(c_d);

    fn value_in_cents(coin : Coin_Data) -> u8 {
        match coin {
            Coin_Data::Penny => 1,
            Coin_Data::Nickel => 5,
            Coin_Data::Dime => 10,
            Coin_Data::Quarter(some_string) => {
                println!("{}", some_string);
                25
            }
        }
    }

    println!("{}", vc);

    // if let..else
    
    if let Some(6) = o {
        println!("Yey!");
    } else{
        println!("No!");
    }

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("idk")
    };
    

}