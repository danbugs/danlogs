use std::collections::HashMap;

fn main(){
    // CREATING AND ADDING
    let mut hm : HashMap<String, i32> = HashMap::new();
    hm.insert("blue".to_string(), 10);
    hm.insert("red".to_string(), 100);
    println!("{:?}", hm);

    // ACCESSING
    // // Method #1 (GET): 
    println!("1: {:?}", hm.get(&"blue".to_string()));

    // // Method #2 ([]): 
    println!("2: {:?}", hm[&"blue".to_string()]);

    for (key, val) in &hm{
        println!("{:?}-{:?}", key, val);
    }

    // UPDATING
    // // Overwriting  
    // hm.insert("blue".to_string(), 1000000);
    // println!("{:?}", hm);

    // // Create iff !existent
    hm.entry("blue".to_string()).or_insert(1000000);
    println!("{:?}", hm);
    hm.entry("yellow".to_string()).or_insert(1); // or_default();
    println!("{:?}", hm);

    // // derefs

}


























// use std::collections::HashMap;

// fn main() {
//     let mut scores : HashMap<String, i32> = HashMap::new();
//     // let mut red = "red".to_string();
//     // let mut red_ref = &red;
//     // scores.insert(red_ref, 50);
//     // println!("{:?}", scores);
//     // println!("{:?}", red_ref);
//     // println!("{:?}", scores.get(&"red".to_string()));
//     scores.entry("yellow".to_string()).or_default();
//     println!("{:?}", scores.get(&"yellow".to_string()));
//     println!("{:?}", scores[&"yellow".to_string()]);
//     println!("{:?}", scores.entry("yellow".to_string()));
//     *scores.entry("yellow".to_string()).or_default() += 1;
//     println!("{:?}", scores);
// }
