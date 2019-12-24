use std::collections::HashMap;

fn main() {
    let mut scores : HashMap<String, i32> = HashMap::new();
    // let mut red = "red".to_string();
    // let mut red_ref = &red;
    // scores.insert(red_ref, 50);
    // println!("{:?}", scores);
    // println!("{:?}", red_ref);
    // println!("{:?}", scores.get(&"red".to_string()));
    scores.entry("yellow".to_string()).or_default();
    println!("{:?}", scores.get(&"yellow".to_string()));
    println!("{:?}", scores[&"yellow".to_string()]);
    println!("{:?}", scores.entry("yellow".to_string()));
    *scores.entry("yellow".to_string()).or_default() += 1;
    println!("{:?}", scores);
}
