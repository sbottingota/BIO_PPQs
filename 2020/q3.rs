const LETTERS: &[u8; 26] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Given a plan, and a limit on the number of adjacent identical characters, returns whether the plan conforms to the limit.
fn is_valid_plan(plan: &Vec<u8>, max_adjacent: &usize) -> bool {
    'outer: for i in 0..plan.len() - max_adjacent {
        for j in 1..*max_adjacent {
            if plan[i] != plan[i + j] {
                continue 'outer
            }
        }

        if plan[i] == plan[i + max_adjacent] {
            return false;
        }
    }

    true
}

/// Returns the nth plan (when ordered in alphabetical order) within the specified parameters. 
pub fn get_nth_plan(n: usize, n_letters: usize, max_adjacent: usize, length: usize) -> Vec<u8> {
    let mut plan: Vec<u8> = vec![0; length];

    let mut count: usize = 0;

    for mut i in 0..n_letters.pow(length as u32) {
        // construct plan
        for j in (0..length).rev() {
            plan[j] = LETTERS[i % n_letters];
            i /= n_letters;
        }

        if is_valid_plan(&plan, &max_adjacent) {
            count += 1;

            if count == n {
                return plan;
            }
        }
    }

    panic!("Plan #{} for n_letters={}, max_adjacent={}, and length={} does not exist. ", n, n_letters, max_adjacent, length);
}

