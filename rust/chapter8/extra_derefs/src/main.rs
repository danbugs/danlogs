use std::collections::HashMap;

fn main() {
    // VECTORS
    let mut v = vec![1, 2, 3];
    println!("{:?}", v);
    for val in &mut v{
        *val += 1;
    }
    println!("{:?}", v);

    // HASHMAPS
    let mut hm = HashMap::new();
    hm.insert("k1", 1);
    hm.insert("k2", 2);
    println!("{:?}", hm);
    for (_, val) in &mut hm{
        *val += 1;
    }
    println!("{:?}", hm);
}
