enum List {
    Node(i32, Box<List>),
    None,
}

use List::*;

fn main() {
    let ll = Node(1, Box::new(Node(2, Box::new(Node(3, Box::new(None))))));
}
