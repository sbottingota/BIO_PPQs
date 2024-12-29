mod q2;
use q2::*;

fn main() {
    println!("{}", Dial::get_pair(1_000_000_000).get_n_letters(6));
}

