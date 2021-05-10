fn main() {
    dbg!(is_palindrome2(121));
    dbg!(is_palindrome2(1231));
    dbg!(is_palindrome2(0));
    dbg!(is_palindrome2(1221));
    dbg!(is_palindrome2(-121));
    dbg!(is_palindrome2(10));
}

pub fn is_palindrome(x : i32) -> bool {
    x.to_string() == x.to_string().chars().rev().collect::<String>()
}

pub fn is_palindrome2(x: i32) -> bool {
    if x < 0 {
        false
    } else {
        let (mut rev, mut num) = (0_i32, x);
        while num > 0 {
            rev = rev * 10 + num % 10;
            num /= 10;
        }
        reverse == x
    }
}