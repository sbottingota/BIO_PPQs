mod q3;
use std::io::stdin;

const ALPHABET: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

fn main() {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);

    let mut buf_parts = buf.split_whitespace();

    let charset_len: usize = buf_parts.next().expect("Invalid input").parse().expect("Invalid input");
    let blockchain_start = buf_parts.next().expect("Invalid input");

    let blockchains = q3::get_blockchains_starting_with(blockchain_start, &ALPHABET[0..charset_len].to_vec());
    println!("{}", blockchains.len());
    // println!("{:?}", blockchains);
}

