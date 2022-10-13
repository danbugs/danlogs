# Aside: `?Sized`

📝 editor: 
```rust,editable
fn main() {
    println!("{}", std::mem::size_of::<i32>());
    println!("{}", std::mem::size_of::<i64>());
    println!("{}", std::mem::size_of::<f32>());
    println!("{}", std::mem::size_of::<f64>());
}
```
🗒️ note #1:
```rust,noplayground
#fn get<str>(&self, k: &str) -> Option<&MyBox<&str>>
#where
#    MyBox<&str>: std::borrow::Borrow<str>,
#    str: Sized + std::cmp::Eq {
#}
```

✅ completed: 
```rust
#fn main() {
#    println!("{}", std::mem::size_of::<i32>());
#    println!("{}", std::mem::size_of::<i64>());
#    println!("{}", std::mem::size_of::<f32>());
#    println!("{}", std::mem::size_of::<f64>());
#
#    assert!(std::mem::size_of::<str>().is_err());
#}
```