use std::{thread, time::Duration, sync::{Mutex, Arc}};

fn print(n: Arc<Mutex<i32>>) {
    for _ in 0..5 {
        let mut n = n.lock().unwrap();
        println!("{} ", n);
        *n += 1;
        thread::sleep(Duration::from_millis(10));
    }
}

fn main() {
    let shared_value = Arc::new(Mutex::new(0));
    let svc = shared_value.clone();

    let t1 = thread::spawn(move || print(shared_value.clone()));
    let t2 = thread::spawn(move || print(svc));
    t1.join().unwrap();
    t2.join().unwrap();
}
