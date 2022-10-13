# Going Back to Our Example from Before - How Do `Borrow` and `AsRef` Differ?

📝 editor: 
```rust,editable
#[derive(Debug)]
struct MyBox<T>(T);

impl std::borrow::Borrow<str> for MyBox<&str> {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
struct FakeHashMap<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K, V> FakeHashMap<K, V> {
    fn new() -> Self {
        Self {
            keys: vec![],
            values: vec![],
        }
    }

    fn insert(&mut self, k: K, v: V) {
        self.keys.push(k);
        self.values.push(v);
    }

    fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: ?Sized + Eq
    {
        let mut found = None;
        for (index, item) in self.keys.iter().enumerate() {
            if item.as_ref() == k {
                found = Some(index);
            }
        }

        if let Some(idx) = found {
            return Some(&self.values[idx]);
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

🗒️ note #1:
```rust,noplayground
#CaseInsensitiveStr("ok") == CaseInsensitiveStr("OK") ✅
```

🗒️ note #2:
```rust,noplayground
#impl Borrow<str> for CaseInsensitiveStr<&str> { 
#    // ..
#} ❓
```

🗒️ note #3:
```rust,noplayground
#CaseInsensitiveStr("ok").borrow() != "OK"
```

✅ completed:
```rust
##[derive(Debug)]
#struct MyBox<T>(T);
#
#impl AsRef<str> for MyBox<&str> {
#    fn as_ref(&self) -> &str {
#        &self.0
#    }
#}
#
#
##[derive(Debug)]
#struct FakeHashMap<K, V> {
#    keys: Vec<K>,
#    values: Vec<V>,
#}
#
#impl<K, V> FakeHashMap<K, V> {
#    fn new() -> Self {
#        Self {
#            keys: vec![],
#            values: vec![],
#        }
#    }
#
#    fn insert(&mut self, k: K, v: V) {
#        self.keys.push(k);
#        self.values.push(v);
#    }
#
#    fn get<Q>(&self, k: &Q) -> Option<&V>
#    where
#        K: AsRef<Q>,
#        Q: ?Sized + Eq
#    {
#        let mut found = None;
#        for (index, item) in self.keys.iter().enumerate() {
#            if item.as_ref() == k {
#                found = Some(index);
#            }
#        }
#
#        if let Some(idx) = found {
#            return Some(&self.values[idx]);
#        }
#
#        None
#    }
#}
#
#fn main() {
#    let mut fhm: FakeHashMap<MyBox<&str>, MyBox<&str>> = FakeHashMap::new();
#    fhm.insert(MyBox("key"), MyBox("value"));
#    println!("{:?}", fhm.get("key"));
#}
```