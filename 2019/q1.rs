pub fn is_palindrome(n: u64) -> bool {
    n.to_string().chars().eq(n.to_string().chars().rev())
}

pub fn next_palindrome_number(mut n: u64) -> u64 {
    loop {
        n += 1;
        if is_palindrome(n) { break }
    }
    n
}

