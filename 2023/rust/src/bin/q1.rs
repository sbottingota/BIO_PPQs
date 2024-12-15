pub fn fibonacci(max: usize) -> Vec<usize> {
    let mut res = Vec::new();

    let mut prev1 = 0;
    let mut prev2 = 1;

    while prev1 + prev2 <= max {
        res.push(prev1 + prev2);
        prev2 = prev1 + prev2;
        prev1 = prev2 - prev1;
    }

    res
}

#[allow(dead_code)]
fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);

    let mut target: usize = buf.trim().parse().unwrap();

    let mut found_number = false;

    for n in fibonacci(target).into_iter().rev() {
        if found_number || n > target {
            found_number = false;
            continue;
        }
        print!("{} ", n);

        target -= n;
        found_number = true;
        if target == 0 {
            break;
        }
    }

    println!();
}

