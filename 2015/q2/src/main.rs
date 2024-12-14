const SHIP_LENGTHS: &[usize] = &[4, 3, 3, 2, 2, 2, 1, 1, 1, 1];

struct Ship {
    pos: (usize, usize),
    horizontal: bool,
}

struct Game {
    grid: Vec<Vec<bool>>, // false = occupied, true = occupied
    ships: Vec<Ship>,
}

impl Game {
    const GRID_SIZE: usize = 10;

    fn new() -> Game {
        Game { grid: vec![vec![false; Self::GRID_SIZE]; Self::GRID_SIZE], ships: Vec::new() }
    }

    #[inline]
    fn can_occupy_square(&self, pos: (usize, usize)) -> bool {
        if pos.0 >= self.grid.len() || pos.1 >= self.grid[0].len() {
            return false;
        }

        for x in [-1_isize, 0, 1] {
            if (pos.0 == 0 && x == -1) || (pos.0 as isize + x) >= self.grid.len() as isize {
                continue;
            }
            for y in [-1_isize, 0, 1] {
                if (pos.1 == 0 && y == -1) || (pos.1 as isize + y) >= self.grid[0].len() as isize {
                    continue;
                }

                if self.grid[(pos.0 as isize + x) as usize][(pos.1 as isize + y) as usize] == true {
                    return false;
                }
            }
        }

        true
    }

    fn can_place(&self, pos: (usize, usize), len: usize, horizontal: bool) -> bool {
        for offset in 0..len {
            if horizontal {
                if !self.can_occupy_square((pos.0 + offset, pos.1)) {
                        return false;
                }
            } else {
                if !self.can_occupy_square((pos.0, pos.1 + offset)) {
                        return false;
                }
            }
        }

        true
    }

    fn place(&mut self, pos: (usize, usize), len: usize, horizontal: bool) {
        for offset in 0..len {
            if horizontal {
                self.grid[pos.0 + offset][pos.1] = true;
            } else {
                self.grid[pos.0][pos.1 + offset] = true;
            }
        }

        self.ships.push(Ship { pos: pos, horizontal: horizontal });
    }

    fn print_ships(&self) {
        for ship in &self.ships {
            println!("{} {} {}", ship.pos.0, ship.pos.1, if ship.horizontal { 'H' } else { 'V' });
        }
    }
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);

    let mut input = buf.split_whitespace().map(|substr| substr.parse::<usize>().unwrap());
    let a = input.next().unwrap();
    let c = input.next().unwrap();
    let m = input.next().unwrap();

    let mut r = 0_usize;

    let mut game = Game::new();

    for &ship_len in SHIP_LENGTHS {
        loop {
            r = (a * r + c) % m;
            let pos = (r % 10, (r / 10) % 10);
            r = (a * r + c) % m;
            if game.can_place(pos, ship_len, r % 2 == 0) {
                game.place(pos, ship_len, r % 2 == 0);
                break;
            }
        }
    }

    game.print_ships();        
}

