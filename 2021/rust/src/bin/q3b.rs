mod q3;
use q3::*;

use itertools::Itertools;

fn main() {
    for permutation in ('A'..='E').permutations(5) {
        if get_n_operations(&permutation) == 6 {
            println!("{}", permutation.into_iter().collect::<String>());
        }
    }
}

