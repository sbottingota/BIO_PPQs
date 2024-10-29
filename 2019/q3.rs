type BlockChain = Vec<char>;

fn get_permutations(len: usize, charset: &Vec<char>) -> Vec<BlockChain> {
    if len == 1 {
        return vec![charset.clone()];
    }

    let mut permutations: Vec<BlockChain> = Vec::new();

    let mut modified_charset;
    for i in 0..charset.len() {
        modified_charset = charset.clone();
        modified_charset.remove(i);
        permutations.extend(get_permutations(len - 1, &modified_charset)
            .into_iter().map(|chain| { 
                let mut v = vec![charset[i]];
                v.extend(chain);
                v
             }));
    }

    permutations
}

fn is_valid_blockchain(chain: &BlockChain) -> bool {
    for i in 0..chain.len() - 2 {
        for j in i + 1..chain.len() - 1 {
            for k in j + 1..chain.len() {
                if chain[i] < chain[j] && chain[j] < chain[k] {
                    return false;
                }
            }
        }
    }

    true
}

pub fn get_blockchains_starting_with(s: &str, charset: &Vec<char>) -> Vec<BlockChain> {
    let mut modified_charset = charset.clone();
    for c1 in s.chars() {
        modified_charset.remove(
            modified_charset.iter().position(|&c2| c1 == c2).expect("Invalid charset")
        );
    }

    get_permutations(modified_charset.len(), &modified_charset)
        .into_iter().map(|chain| {
            let mut v = s.chars().collect::<Vec<_>>();
            v.extend(chain);
            v
        })
        .filter(|chain| is_valid_blockchain(chain))
        .collect::<Vec<BlockChain>>()
}

