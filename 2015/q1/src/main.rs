pub fn group(word: &[char]) -> Vec<Vec<Vec<char>>> {
    let mut groups = vec![vec![word.to_vec()]];

    for partition_len in 1..word.len() {
        groups.append(&mut group(&word[partition_len..])
            .into_iter()
            .map(|mut vec| { 
                vec.insert(0, word[..partition_len].to_vec());
                vec 
            })
            .collect()
        );
    }

    groups
}

pub fn is_palindrome(partition: &[Vec<char>]) -> bool {
    for i in 0..partition.len() / 2 {
        if partition[i] != partition[partition.len() - i - 1] {
            return false;
        }
    }
    true
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
    let word: Vec<char> = buf[..buf.len() - 1].chars().collect();
    println!("{}", group(&word)
        .into_iter()
        .filter(|partition| partition.len() > 1 && is_palindrome(partition))
        .count());
}

