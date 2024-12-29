#![allow(dead_code)]

#[derive(Debug)]
pub struct Dial {
    letters: Vec<char>,
}

impl Dial {
    fn new() -> Self {
        Self { letters: Vec::new() }
    }

    pub fn get_pair(n: usize) -> Self {
        let mut dial = Self::new();

        let mut alphabet: Vec<_> = ('A'..='Z').collect();

        let mut i = 0_usize;

        while !alphabet.is_empty() {
            i = (i + n - 1) % alphabet.len();

            dial.letters.push(alphabet.remove(i));
        }

        dial
    }

    fn encode(&self, msg: &[char]) -> Vec<char> {
        let mut msg = msg.to_vec();

        let mut i = 0_usize;

        for j in 0..msg.len() {
            msg[j] = self.letters[(i + msg[j] as usize - 'A' as usize) % self.letters.len()];
            i += 1;
        }

        msg
    }

    pub fn get_n_letters(&self, n: usize) -> String {
        self.letters.iter().take(n).collect::<String>()
    }
}

fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    let n: usize = buf.split_whitespace().next().unwrap().parse().unwrap();
    let msg: Vec<_> = buf.split_whitespace().nth(1).unwrap().chars().collect();

    let dial = Dial::get_pair(n);

    println!("{}", dial.get_n_letters(6));

    println!("{}", dial.encode(&msg).into_iter().collect::<String>());

    Ok(())
}

