mod q2;
use q2::IntList::*;

fn parse(s: String) -> q2::IntList {
    let mut ret = Null;

    let mut i: usize = 0;
    while i < s.len() {
        ret = match s.chars().nth(i).unwrap() {
            'E' => ret * E,
            'O' => ret * O,
            'T' => ret * T,
            '(' => {
                let end_bracket_index = s.len() - s.chars().rev()
                    .position(|c| c == ')')
                    .unwrap() - 1;

                let new_val = ret * parse(s[i+1..end_bracket_index].to_string());
                i += end_bracket_index;
                new_val

            },
            c => panic!("Invalid character '{}'", c)
        };

        i += 1;
    }

    ret
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
    let mut iter = buf.split_whitespace();

    let s: String = iter.next().unwrap().to_string();
    let i: usize = iter.next().unwrap().parse().unwrap();

    println!("{}", parse(s).index(i));
}

