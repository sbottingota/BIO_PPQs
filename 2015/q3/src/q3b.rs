use itertools::Itertools;
mod main;

fn main() {
    let target: Vec<char> = "AABCCBDD".chars().collect();
    let mut target_sorted = target.clone();
    target_sorted.sort();

    for (i, permutation) in get_arrangements!(target_sorted).enumerate() {
        if *permutation.into_iter().copied().collect::<Vec<char>>() == target {
            println!("{}", i + 1);
        }
    }
}

