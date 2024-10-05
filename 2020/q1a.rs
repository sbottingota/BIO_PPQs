mod q1;

use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf);

    let mut numeral = String::from(buf.split_whitespace().nth(0).unwrap());
    let n = buf.split_whitespace().nth(1).unwrap().parse::<u32>().unwrap();

    for i in 0..n {
        numeral = q1::look_and_say(&numeral);
    }

    println!("{} {}", 
        numeral.chars().filter(|c| *c == 'I').count(),
        numeral.chars().filter(|c| *c == 'V').count()
    );
}

