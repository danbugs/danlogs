#![allow(dead_code)]

use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    val: i32,
    points_to: Option<Weak<RefCell<Node>>>,
}

fn main() {
    let n2 = Rc::new(RefCell::new(Node { val: 2, points_to: None }));
    let n1 = Rc::new(RefCell::new(Node { val: 1, points_to: Some(Rc::downgrade(&n2)) }));
    n2.borrow_mut().points_to = Some(Rc::downgrade(&n1));
    dbg!(Rc::weak_count(&n1));
    dbg!(Rc::weak_count(&n2));
    // std::mem::drop(n1);
    dbg!(Rc::weak_count(&n2));
    dbg!(&n2);

    dbg!(n2.borrow().points_to.as_ref().unwrap().upgrade());
}