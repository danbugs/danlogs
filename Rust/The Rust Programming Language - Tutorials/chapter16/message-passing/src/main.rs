use std::thread;

fn main() {
    let (tx, rx) = crossbeam_channel::unbounded();

    let tx2 = tx.clone();
    let rx2 = rx.clone();

    let t1 = thread::spawn(move || {
        println!("rx2 got: {:#?}", rx2.try_recv());
    });

    tx2.send("Bye!".to_string()).unwrap();

    let t2 = thread::spawn(move || {
        let val = String::from("Hello, World!");
        tx.send(val).unwrap();
        // println!("val is {}", val);
    });

    println!("rx got: {}", rx.recv().unwrap());
    
    t1.join().unwrap();
    t2.join().unwrap();
}




























// BOUNDED EXAMPLE:
// use std::thread;

// fn main() {
//     let (tx, rx) = crossbeam_channel::bounded(1);

//     let t = thread::spawn(move || {
//         tx.send("Hello,").unwrap();
//         tx.send("World!").unwrap();
//     });

//     t.join().unwrap();

//     for _ in 0..2 {
//         print!("{} ", rx.recv().unwrap());
//     }
//     println!("");
// }