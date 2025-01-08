fn get_nth_digit(start: usize, mut n: usize) -> char {
    let mut digits: Vec<char>;

    for i in start.. {
        digits = i.to_string().chars().collect();

        if digits.len() < n {
            n -= digits.len()
        } else {
            return digits[n - 1];
        }
    }

    unreachable!()
}

fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    let input: Vec<usize> = buf.split_whitespace().map(|substr| substr.parse().unwrap()).collect();

    println!("{}", get_nth_digit(input[0], input[1]));

    Ok(())
}

