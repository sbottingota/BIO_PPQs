mod main;
use main::*;

fn main() {
    for grouping in  group(&['A', 'A', 'B', 'C', 'B', 'A', 'A'])
        .into_iter()
        .filter(|partition| partition.len() > 1 && is_palindrome(partition))
        .collect::<Vec<_>>() {
        println!("{:?}", grouping);
    }
}

