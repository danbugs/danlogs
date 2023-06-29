macro_rules! m {
    ($($s:stmt)*) => {
        $(
            { stringify!($s); 1 }
        )<<*
    };
}

// Statement (stmt) Examples:
// 1. `let x = 10`;
// 2. `x = x + 1;`
// 3. `println!("Hello, world!");`

fn main() {
    print!(
        "{}{}{}",
        m! { return || true },
        // return closure true = 1 stmt
        m! { (return) || true },
        // return OR true = 1 stmt
        m! { {return} || true },
        // return in block
        // closure true
        // = 2 stmts
        // 1 << 1 = 2
    );
}