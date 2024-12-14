use itertools::Itertools;

#[macro_export]
macro_rules! get_arrangements {
    ($art:expr) => {
        $art.iter()
                .permutations($art.len())
                .unique()
    }
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
    let input_ints = buf.split_whitespace().map(|substr| substr.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let mut art: Vec<char> = Vec::new();
    for (i, &n) in input_ints[..4].into_iter().enumerate() {
        for _ in 0..n {
            art.push(['A', 'B', 'C', 'D', 'E'][i]);
        }
    }

    let permutation = get_arrangements!(art).nth(input_ints[4] + 1).unwrap(); // + 1 because of 1 based indexing
    println!("{}", permutation.into_iter().join(""));
}

