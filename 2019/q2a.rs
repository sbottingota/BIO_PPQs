mod q2;
use std::io::stdin;

fn main() {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);

    let mut args = buf.trim().split_whitespace();

    let trail_length = args.next().unwrap().parse::<usize>().unwrap();
    let steps_chars = args.next().unwrap().chars();
    let n_moves = args.next().unwrap().parse::<usize>().unwrap();

    let mut explorer = q2::Explorer::new(trail_length);
    let steps = steps_chars.map(|c| {
        match c {
            'F' => q2::Direction::Forward,
            'R' => q2::Direction::Right,
            'L' => q2::Direction::Left,
            _   => panic!("Invalid input")
        }
    }).collect::<Vec<q2::Direction>>();

    for i in 0..n_moves {
        let res = explorer.step(steps[i % steps.len()]);
        if res.is_err() {
            break;
        }
    }

    println!("({}, {})", explorer.pos.0, explorer.pos.1);
}

