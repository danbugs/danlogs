use std::thread;
use std::sync::Arc;
use std::fmt::Debug;

fn pair_print_pair<T: Debug + Send + Sync + 'static>(value1: Arc<T>, value2: Arc<T>) {
    let v1c = value1.clone();
    let v2c = value2.clone();

    let t = thread::spawn(move || {
        println!("Value 1: {:?}", *v1c);
        println!("Value 2: {:?}", *v2c);
    });

    let t1 = thread::spawn(move || {
        println!("Value 1: {:?}", *value1);
        println!("Value 2: {:?}", *value2);
    });

    t.join().unwrap();
    t1.join().unwrap();
}

fn main() {
    let value1 = Arc::new("Hello");
    let value2 = Arc::new("World");

    pair_print_pair(value1, value2);
}
