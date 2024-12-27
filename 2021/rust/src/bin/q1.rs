#![allow(dead_code)]

pub fn is_pat(s: &str) -> bool {
    if s.len() == 1 {
        return true;
    }

    'outer: for partition in 1..s.len() {
        for c1 in s[..partition].chars() {
            for c2 in s[partition..].chars() {
                if c1 <= c2 {
                    continue 'outer;
                }
            }
        }

        if is_pat(&s[..partition].chars().rev().collect::<String>()) && is_pat(&s[partition..].chars().rev().collect::<String>()) {
            return true;
        }
    }

    false
}

#[inline]
fn to_yes_no(b: bool) -> &'static str {
    if b { "YES" } else { "NO" }
}

fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    let s1 = buf.split_whitespace().next().unwrap().to_string();
    let s2 = buf.split_whitespace().nth(1).unwrap().to_string();
    let mut s3 = s1.clone();
    s3.push_str(&s2);

    println!("{}", to_yes_no(is_pat(&s1)));
    println!("{}", to_yes_no(is_pat(&s2)));
    println!("{}", to_yes_no(is_pat(&s3)));

    Ok(())
}

