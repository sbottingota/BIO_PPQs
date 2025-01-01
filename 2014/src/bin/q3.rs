use itertools::Itertools;

const N_PASSWORD_CHARS: usize = 36;

fn id_to_char(id: usize) -> char {
    ('A'..='Z').chain('0'..='9').nth(id).expect(&format!("Invalid id '{}', must be between 0 and {} inclusive", id, N_PASSWORD_CHARS - 1))
}

fn get_nth_password(index: usize) -> Vec<usize> {
    let mut i = 0_usize;

    for j in 1.. {
        for combination in (0..N_PASSWORD_CHARS).combinations(j) {
            i += 1;
            if i == index {
                return combination;
            }
        }
    }

    panic!("Error finding password {}", index)
}

fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    let index = buf.trim().parse().unwrap();
    let password: String = get_nth_password(index).into_iter().map(|id| id_to_char(id)).collect();
    
    println!("{}", password);

    Ok(())
}

