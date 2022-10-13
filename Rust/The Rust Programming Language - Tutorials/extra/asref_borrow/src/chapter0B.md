### cont'd: Implementing `Borrow` for Our Own Type, and Seeing Its' Utility

📝 editor: 
```rust,editable
#[derive(Debug)]
struct MyBox<T>(T);
impl std::borrow::Borrow<str> for MyBox<&str> {
    fn borrow(&self) -> &str {
        &self.0
    }
}

struct FakeHashMap<K, V> {
    keys: Vec<K>,
    values: Vec<V>
}

impl<K, V> FakeHashMap<K, V> {
    fn new() -> Self {
        Self {
            keys: vec![],
            values: vec![]
        }
    }

    fn insert(&mut self, key: K, value: V) {
        self.keys.push(key);
        self.values.push(value);
    }

    fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::cmp::Eq {
            let mut found = None;

            for (index, item) in self.keys.iter().enumerate() {
                if item.borrow() == k {
                    found = Some(index);
                    break;
                }
            }

            if let Some(index) = found {
                return Some(&self.values[index]);
            }

            None
        }

}

fn main() {
    let mut fhm: FakeHashMap<MyBox<&str>, MyBox<&str>> = FakeHashMap::new();
    fhm.insert(MyBox("key"), MyBox("value"));
    println!("{:?}", fhm.get("key"));
}
```

✅ completed: 
```rust
##[derive(Debug)]
#struct MyBox<T>(T);
#
#impl std::borrow::Borrow<str> for MyBox<&str> {
#    fn borrow(&self) -> &str {
#        &self.0
#    }
#}
#
#struct FakeHashMap<K, V> {
#    keys: Vec<K>,
#    values: Vec<V>
#
#}
#impl<K, V> FakeHashMap<K, V> {
#    fn new() -> Self {
#        Self {
#            keys: vec![],
#            values: vec![]
#        }
#    }
#
#    fn insert(&mut self, key: K, value: V) {
#        self.keys.push(key);
#        self.values.push(value);
#    }
#
#    fn get<Q>(&self, k: &Q) -> Option<&V>
#    where
#        K: std::borrow::Borrow<Q>,
#        Q: ?Sized + std::cmp::Eq {
#            let mut found = None;
#
#            for (index, item) in self.keys.iter().enumerate() {
#                if item.borrow() == k {
#                    found = Some(index);
#                    break;
#                }
#            }
#
#            if let Some(index) = found {
#                return Some(&self.values[index]);
#            }
#
#            None
#        }
#
#}
#
#fn main() {
#    let mut fhm: FakeHashMap<MyBox<&str>, MyBox<&str>> = FakeHashMap::new();
#    fhm.insert(MyBox("key"), MyBox("value"));
#    println!("{:?}", fhm.get("key"));
#}
```