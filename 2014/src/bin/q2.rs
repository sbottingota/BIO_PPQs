use std::collections::HashSet;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
enum Color {
    Red,
    Green,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct Connection {
    center: (usize, usize),
    ends: [(usize, usize); 2],
    color: Color,
}

impl Connection {
    fn new(end1: (usize, usize), center: (usize, usize), end2: (usize, usize), color: Color) -> Self {
        Self { center, ends: [end1, end2], color }
    }

    fn matches(&self, other: &Self) -> bool {
        self.ends.contains(&other.center) && other.ends.contains(&self.center) && self.color == other.color
    }
}

fn find_loop_from(connections: &[Connection], start: &Connection) -> Option<Vec<Connection>> {
    let mut ret = Vec::from([start.clone()]);

    loop {
        let prev = ret.last().unwrap().clone();
        if let Some(next) = connections.iter().find(|&connection| prev.matches(connection) && !ret[1..].contains(connection) && !(ret.len() == 2 && start == connection)) {
            if next == start {
                return Some(ret);
            }
            ret.push(next.clone());
        } else {
            return None;
        }
    }
}

fn find_all_loops(connections: &[Connection]) -> Vec<Vec<Connection>> {
    let mut loops = HashSet::new();
    for connection in connections {
        if let Some(mut connection_loop) = find_loop_from(connections, connection) {
            connection_loop.sort();
            loops.insert(connection_loop);
        }
    }

    loops.into_iter().collect()
}

fn grid_to_connections(grid: &[Vec<usize>]) -> Vec<Connection> {
    let mut connections = Vec::new();
    for (x, row) in grid.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            match tile {
                1 => {
                    if x > 0 && x < grid.len() - 1 {
                        connections.push(Connection::new((x + 1, y), (x, y), (x - 1, y), Color::Red));
                    }
                    if y > 0 && y < grid[0].len() - 1 {
                        connections.push(Connection::new((x, y + 1), (x, y), (x, y - 1), Color::Green));
                    }
                },

                2 => {
                    if x > 0 && x < grid.len() - 1 {
                        connections.push(Connection::new((x + 1, y), (x, y), (x - 1, y), Color::Green));
                    }
                    if y > 0 && y < grid[0].len() - 1 {
                        connections.push(Connection::new((x, y + 1), (x, y), (x, y - 1), Color::Red));
                    }
                },

                3 => {
                    if x > 0 && y > 0 {
                        connections.push(Connection::new((x - 1, y), (x, y), (x, y - 1), Color::Red));
                    }
                    if x < grid.len() - 1 && y < grid[0].len() - 1 {
                        connections.push(Connection::new((x + 1, y), (x, y), (x, y + 1), Color::Green));
                    }
                },

                4 => {
                    if x > 0 && y < grid[0].len() - 1 {
                        connections.push(Connection::new((x - 1, y), (x, y), (x, y + 1), Color::Red));
                    }
                    if x < grid.len() && y > 0 {
                        connections.push(Connection::new((x + 1, y), (x, y), (x, y - 1), Color::Green));
                    }
                },

                5 => {
                    if x > 0 && y > 0 {
                        connections.push(Connection::new((x - 1, y), (x, y), (x, y - 1), Color::Green));
                    }
                    if x < grid.len() - 1 && y < grid[0].len() - 1 {
                        connections.push(Connection::new((x + 1, y), (x, y), (x, y + 1), Color::Red));
                    }
                },

                6 => {
                    if x > 0 && y < grid[0].len() - 1 {
                        connections.push(Connection::new((x - 1, y), (x, y), (x, y + 1), Color::Green));
                    }
                    if x < grid.len() && y > 0 {
                        connections.push(Connection::new((x + 1, y), (x, y), (x, y - 1), Color::Red));
                    }
                },

                _ => panic!("Invalid tile id '{}'", tile),
            }
        }
    }
    connections
}

fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut buf)?;
    let grid_size: usize = buf.trim().parse().unwrap();

    let mut grid: Vec<Vec<usize>> = Vec::new();

    for _ in 0..grid_size {
        buf.clear();
        stdin.read_line(&mut buf)?;
        grid.push(buf.split_whitespace().map(|substr| substr.parse().unwrap()).collect());
    }

    let connections = grid_to_connections(&grid);

    for color in [Color::Red, Color::Green] {
        print!("{} ", find_all_loops(&connections.iter().filter(|connection| connection.color == color).cloned().collect::<Vec<_>>()).into_iter().flatten().count());
    }
    println!();

    Ok(())
}

