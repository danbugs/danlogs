# The `Borrow` Trait

📝 editor: 
```rust,editable
use std::collections::HashMap;

fn main() {
}
```

🗒️ note #1:
```rust,noplayground
#impl Borrow<str> for String {
#    fn borrow(&self) -> &str {
#        &self[..]
#    }
#}
```

🗒️ note #2:
```rust,noplayground
#impl<K, V> HashMap<K, V> {
# fn get<Q>(&self, k: &Q) 
#     -> Option<&V>
#     where K: Borrow<Q> { }
#}
```

✅ completed:
```rust
#use std::collections::HashMap;

#fn main() {
#    let mut hm: HashMap<String, String> = HashMap::new();
#     hm.insert("key".to_string(), "value".to_string());
#     println!("{:?}", hm.get("key"));
#     // how can we search hm with a &str (i.e., a string slice) rather than a String
#}
```