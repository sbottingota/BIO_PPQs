mod q1;
use q1::*;

use itertools::Itertools;

fn main() {
    for permutation in ('A'..='D').permutations(4).map(|v| v.into_iter().collect::<String>()) {
        if is_pat(&permutation) {
            println!("{}", permutation);
        }
    }
}

