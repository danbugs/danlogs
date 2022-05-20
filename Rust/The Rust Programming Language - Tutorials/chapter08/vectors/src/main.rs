fn main() {
    // VECTORS
    // https://doc.rust-lang.org/std/collections/index.html
    let mut v : Vec<i32> = Vec::new();
    let mut v1 = vec![1, 2, 3];
    // ADD
    v.push(5);
    v.push(1);
    println!("{:?}", &v);

    // REMOVE
    v.pop(); // LIFO
    println!("{:?}", &v);
    v.push(5);
    v.push(1);
    v.push(5);
    v.push(1);
    println!("{:?}", &v);
    v.remove(0);
    println!("{:?}", &v);

    // GET
    //println!("{:}", &v[100]);
    println!("{:?}", v.get(100));

    // Be careful!
    // let r = &v1[0];
    // v1.push(6);
    // println!("{:?}", r);

    // ITERATING
    // for i in &v{
    //     println!("{:?}", i);
    // }
    for i in (0..v.len()){
        println!("{:?}", &v[i]);
    }

    // Vectors of different types?
    #[derive(Debug)]
    enum TYPE{
        INTEGER(i32),
        STRING(String),
    }

    let mut v2 = Vec::new();
    v2.push(TYPE::INTEGER(3));
    v2.push(TYPE::STRING(String::from("ok")));

    println!("{:?}", v2);

    match &v2[0]{
        TYPE::INTEGER(i) => println!("{}", i),
        TYPE::STRING(s) => println!("{}", s),
    }
































    // let mut v : Vec<u8> = Vec::new();
    // v.push(5);
    // v.push(1);
    // println!("{:?}", v);
    // v.pop();
    // println!("{:?}", v);

    // match v.get(0){
    //     Some(value) => println!("{}", value),
    //     None => println!("none")
    // }

    // println!("{:}", &v[0]);

    // // let mut v1 = vec![1,2,3,4,5];
    // // let r = &v1[4];
    // // v1.pop();
    // // println!("{:}",r);

    // // MISC

    // v.push(1);
    // v.push(3);
    // println!("{:?}", &v);
    // v.reverse();
    // println!("{:?}", &v);
}