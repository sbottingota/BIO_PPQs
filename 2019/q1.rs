pub fn is_palindrome(mut n: u64) -> bool {
    let mut rev_n = n.to_string().chars().rev().collect::<String>().parse::<u64>().unwrap();

    for _ in 0..=n.ilog10() / 2 {
        if n % 10 != rev_n % 10 {
            return false;
        }
        n /= 10;
        rev_n /= 10;
    }
    true
}

pub fn next_palindrome_number(mut n: u64) -> u64 {
    loop {
        n += 1;
        if is_palindrome(n) { break }
    }
    n
}

