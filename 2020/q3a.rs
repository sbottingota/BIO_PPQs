mod q3;

use std::io;

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);

    let mut buf_iter = buf.split_whitespace();

    let n_letters = buf_iter.next().unwrap().parse::<usize>().unwrap();
    let max_adjacent = buf_iter.next().unwrap().parse::<usize>().unwrap();
    let length = buf_iter.next().unwrap().parse::<usize>().unwrap();

    buf.clear();
    let _ = io::stdin().read_line(&mut buf);
    let n = buf.trim_end().parse::<usize>().unwrap();

    println!("{}", String::from_utf8(q3::get_nth_plan(n, n_letters, max_adjacent, length)).unwrap());
}
