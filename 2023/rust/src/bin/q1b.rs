mod q1;

fn main() {
    println!("{}", q1::fibonacci(1_000_000).into_iter().rev().next().unwrap());
}

