use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicI32, Ordering};

fn print(n: &AtomicI32) {
    for _ in 0..5 {
        let value = n.fetch_add(1, Ordering::SeqCst);
        println!("{} ", value);
        thread::sleep(Duration::from_millis(10));
    }
}

fn main() {
    static SHARED_VALUE: AtomicI32 = AtomicI32::new(0);

    let t1 = thread::spawn(|| print(&SHARED_VALUE));
    let t2 = thread::spawn(|| print(&SHARED_VALUE));

    t1.join().unwrap();
    t2.join().unwrap();
}
