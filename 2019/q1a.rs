mod q1;

use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Invalid input");
    
    println!("{}", q1::next_palindrome_number(s.trim().parse().expect("Invalid input")));
}

