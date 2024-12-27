mod q1;
use q1::*;

use itertools::Itertools;

fn main() {
    println!("{}", ('A'..='Z').permutations(26).map(|v| v.into_iter().collect::<String>()).filter(|s| is_pat(s)).count());
}

