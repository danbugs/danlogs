# The `AsRef` Trait

🗒️ note #1:
```rust,noplayground
#impl AsRef<[u8]> for str {
#    // [..]
#}
#
#impl AsRef<[u8]> for String {
#    // [..]
#}
#
#impl AsRef<str> for String {
#    // [..]
#}
```

🗒️ note #2:
```rust,noplayground
#pub trait AsRef<T> 
#where
#    T: ?Sized, 
#{
#    fn as_ref(&self) -> &T;
#}
#
#pub trait Borrow<Borrowed> 
#where
#    Borrowed: ?Sized, 
#{
#    fn borrow(&self) -> &Borrowed;
#}
```