fn main() {
    // CREATING
    let mut s = String::from("hello");
    let s1 = String::new();
    let s2 = "hello".to_string();
    println!("{}, {}", s, s2);

    // UPDATING
    println!("{}", s);
    s.push_str(", world");
    println!("{}", s);
    s.push('!');
    println!("{}", s);

    // CONCATENATING
    // +
    let s3 = "1".to_string();
    let s4 = "2".to_string();
    // let s5 = s3 + &s4;
    // fn add(self, s : &str) -> String{...} &String
    // format!
    let s5 = format!("{}, {}", s3, s4);
    println!("{}", s5);



    // INDEXING?
    // s[0]
    // д 
    println!("{:?}", "д".to_string().bytes());
    println!("{:?}", "h".to_string().bytes());
    // д = s[0..2]
    // h = s[0..1]
    println!("{:?}", &s2[0..1]);

    // ITERATING
    for b in s2.bytes(){
        println!("{}", b);
    }

    for c in s2.chars(){
        println!("{}", c);
    }


















//     test(&"ok".to_string());
//     let s1 = "ok".to_string();
//     let s2 = ", lol".to_string();
//     let s3 =  s1 + &s2;
//     println!("{}", s3);
//     let s4 = "hello".to_string();
//     println!("{}", &s4[0..1]);

//     for c in s4.chars(){
//         println!("{}", c);
//     }

//     for b in s4.bytes(){
//         println!("{}", b);
//     }
}

// fn test(s : &str){
//     println!("got {}", s);
// }