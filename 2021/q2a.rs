mod q2;

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    let _ = stdin.read_line(&mut buf);
    let mut line_iter = buf.split_whitespace();
    let n_players = line_iter.next().unwrap().parse::<usize>().unwrap();
    let n_moves = line_iter.next().unwrap().parse::<usize>().unwrap();

    let _ = stdin.read_line(&mut buf);
    line_iter = buf.split_whitespace();
    let mut players: Vec<q2::Player> = Vec::new();
    for i in 0..n_players {
        players.push(q2::Player::new(i as isize + 1, line_iter.next().unwrap().parse::<usize>().unwrap()));
    }

    let mut board = q2::Board::new();

    for _ in 0..n_moves {
        for i in 0..players.len() {
            println!("player {} turn", i + 1);
            for j in 0..players.len() {
                println!("player {} pos: {:?}", j + 1, players[j].pos);
            }

            board.print();
            if board.needs_enlargement() {
                board.enlarge();
                for j in 0..players.len() { // reposition players in enlarged board
                    players[j].pos[0] += 1;
                    players[j].pos[1] += 1;
                }
            }

            players[i].make_move(&mut board);

            for j in 0..players.len() {
                if players[j].needs_repositioning(&board) {
                    players[j].reposition(&board);
                }
            }
        }
    }

    for player in players {
        println!("{}", player.points);
    }
}

