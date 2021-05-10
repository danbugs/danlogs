fn main() {
    let v1 = vec![1, 2, 3];
    let v2 : Vec<_> = v1.iter().map(|val| val + 1).collect();
    println!("{:?}", v2);
}