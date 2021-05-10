pub fn print_stuff() {
    println!("stuff")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_stuff() {
        print_stuff();
    }
}
