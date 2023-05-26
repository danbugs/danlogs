use std::thread;
use std::time::Duration;

fn main() {
    let v: Vec<i32> = vec![1, 2, 3];

    let t1 = thread::spawn(|| {
        // print vec
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // oh no!

    // waits for t1 to finish
    t1.join().unwrap();
}































// WORKING VERSION:
// use std::thread;
// use std::time::Duration;
// use std::sync::{Mutex, Arc};

// fn print(n: Arc<Mutex<i32>>) {
//     for _ in 0..5 {
//         let mut lock = n.lock().unwrap();
//         println!("{} ", lock);
//         *lock += 1;
//         thread::sleep(Duration::from_millis(10));
//     }
// }

// fn main() {
//     let shared_value = Arc::new(Mutex::new(0));
//     let svc = shared_value.clone();

//     let t1 = thread::spawn(move || print(shared_value.clone()));
//     let t2 = thread::spawn(move || print(svc));

//     t1.join().unwrap();
//     t2.join().unwrap();
// }
