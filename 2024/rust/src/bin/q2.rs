type List = Box<dyn Fn(usize) -> usize>;

fn e(n: usize) -> usize {
    2 * n
}

fn o(n: usize) -> usize {
    2 * n - 1
}

fn t(mut n: usize) -> usize {
    let mut i = 1;
    loop {
        for _ in 0..i {
            n -= 1;

            if n == 0 {
                return i;
            }
        }
        i += 1;
    }
}

fn combine(l1: List, l2: List) -> List {
    Box::new(move |n| l2(l1(l2(n))))
}

fn char_to_list(c: char) -> List {
    Box::new(
        match c {
            'E' => e,
            'O' => o,
            'T' => t,
            _ => panic!("Invalid list id '{}'", c),
        }
    )
}

fn parse(s: &[char]) -> Box<dyn Fn(usize) -> usize> {
    let mut ret: Option<List> = None;

    let mut i = 0_usize;
    while i < s.len() {
        if s[i] == '(' {
            let mut bracket_count = 1;

            for (j, c) in s.iter().enumerate().skip(i + 1) {
                if *c == '(' {
                    bracket_count += 1;
                } else if *c == ')' {
                    bracket_count -= 1;
                }

                if bracket_count == 0 {
                    ret = Some(if let Some(f) = ret {
                        combine(f, parse(&s[i+1..j]))
                    } else {
                        parse(&s[i+1..j])
                    });

                    i = j;
                    break;
                }
            }
        } else if s[i] != ')' {
            ret = Some(if let Some(f) = ret {
                combine(f, char_to_list(s[i]))
            } else {
                char_to_list(s[i])
            });
        }

        i += 1;
    }

    ret.expect("Invalid input")
}

fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    let list = parse(&buf.split_whitespace().next().unwrap().chars().collect::<Vec<_>>());
    let index: usize = buf.split_whitespace().nth(1).unwrap().parse().unwrap();

    println!("{}", list(index));
    
    Ok(())
}

