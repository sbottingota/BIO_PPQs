use std::collections::VecDeque;

type Edge = [usize; 4];

pub struct Board {
    pub grid: VecDeque<VecDeque<[isize; 2]>>,
}

impl Board {
    const UNFILLED: isize = -1;

    const SCORE_CONFIGURATIONS: [[[isize; 2]; 2]; 3] = [
        [[1, 0], [1, 1]],
        [[-1, 0], [0, 1]],
        [[0, -1], [-1, -1]],
    ];

    const TRIANGLE_CONFIGURATIONS: [[[[isize; 3]; 6]; 3]; 2] = [
        [
            [[0, 0, 0], [0, 0, 1], [-1, 0, 0], [-1, -1, 1], [-1, -1, 0], [0, -1, 1]],
            [[0, 0, 0], [0, -1, 1], [0, -1, 0], [1, -1, 1], [1, 0, 0], [1, 0, 1]],
            [[0, 0, 0], [1, 0, 1], [1, 1, 0], [1, 1, 1], [0, 1, 0], [0, 0, 1]]
        ],
        [
            [[0, 0, 1], [-1, 0, 1], [-1, -1, 1], [-1, -1, 0], [0, -1, 1], [0, 0, 0]],
            [[0, 0, 1], [0, 1, 0], [0, 1, 1], [-1, 1, 0], [-1, 0, 1], [-1, 0, 0]],
            [[0, 0, 1], [0, 0, 0], [1, 0, 1], [1, 1, 0], [1, 1, 1], [0, 1, 0]]
        ],
    ];

    const EDGE_CONFIGURATIONS: [[[isize; 5]; 3]; 2] = [
        [
            [2, 1, 0, 0, 1],
            [0, 0, 1, 2, 2],
            [1, 2, 2, 1, 0]
        ],
        [
            [1, 0, 0, 1, 2],
            [2, 2, 1, 0, 0],
            [0, 1, 2, 2, 1]
        ]
    ];

    pub fn new() -> Self {
        let mut board = Board { grid: VecDeque::new() };
        board.grid.push_front(VecDeque::new());
        board.grid[0].push_front([Self::UNFILLED, 0]);
        /*
        for _ in 0..2 {
            board.enlarge();
        }
        */
        board
    }

    pub fn enlarge(&mut self) {
        for row in &mut self.grid {
            row.push_front([Self::UNFILLED, Self::UNFILLED]);
            row.push_back([Self::UNFILLED, Self::UNFILLED]);
        }

        self.grid.push_front(VecDeque::new());
        self.grid.push_back(VecDeque::new());

        let grid_len = self.grid.len();
        for _ in 0..self.grid[1].len() {
            self.grid[0].push_front([Self::UNFILLED, Self::UNFILLED]);
            self.grid[grid_len - 1].push_front([Self::UNFILLED, Self::UNFILLED]);
        }
    }

    pub fn needs_enlargement(&self) -> bool {
        // top and bottom
        for square in &self.grid[0] {
            for cell in square {
                if *cell != Self::UNFILLED {
                    return true;
                }
            }
        }
        for square in &self.grid[self.grid.len() - 1] {
            for cell in square {
                if *cell != Self::UNFILLED {
                    return true;
                }
            }
        }
        for square in &self.grid[1] {
            for cell in square {
                if *cell != Self::UNFILLED {
                    return true;
                }
            }
        }
        for square in &self.grid[self.grid.len() - 2] {
            for cell in square {
                if *cell != Self::UNFILLED {
                    return true;
                }
            }
        }

        // sides
        for i in 0..self.grid.len() {
            for cell in self.grid[i][0] {
                if cell != Self::UNFILLED {
                    return true;
                }
            }
            for cell in self.grid[i][self.grid[i].len() - 1] {
                if cell != Self::UNFILLED {
                    return true;
                }
            }
            for cell in self.grid[i][1] {
                if cell != Self::UNFILLED {
                    return true;
                }
            }
            for cell in self.grid[i][self.grid[i].len() - 2] {
                if cell != Self::UNFILLED {
                    return true;
                }
            }
        }

        false
    }

    fn get_next_edge_from(&self, prev: Edge) -> Edge {
        let triangle_config = Self::TRIANGLE_CONFIGURATIONS[prev[2]][prev[3]];
        let edge_config = Self::EDGE_CONFIGURATIONS[prev[2]][prev[3]];

        'outer: for i in 0..edge_config.len() {

            /*
            let indices = [configs[i+1][0], configs
            for index in indices {
                if index < 0 {
                    continue 'outer;
                }
            }
            */

            if self.grid[(prev[0] as isize + triangle_config[i+1][0]) as usize][(prev[1] as isize + triangle_config[i+1][1]) as usize][triangle_config[i+1][2] as usize] == Self::UNFILLED {

                return [(prev[0] as isize + triangle_config[i][0]) as usize, (prev[1] as isize + triangle_config[i][1]) as usize, triangle_config[i][2] as usize, edge_config[i] as usize];
            }
        }

        panic!("No next edge found (possible problem with player relocation)");
    }

    fn get_edge_path_from(&self, start: Edge) -> Vec<Edge> {
        let mut vertices: Vec<Edge> = Vec::new();

        let mut current = start;
        loop {
            current = self.get_next_edge_from(current);

            if vertices.iter().filter(|&x| *x == current).count() > 0 {
                return vertices.into_iter().rev().collect();
            }

            vertices.push(current);
        }
    }

    fn can_score_point(&self, pos: &Edge, id: isize) -> bool {
        let mult: isize = if pos[3] == 0 { 1 } else { -1 };

        'outer: for config in Self::SCORE_CONFIGURATIONS {
            let indices = [(config[0][0] * mult + pos[0] as isize), (config[0][1] * mult + pos[1] as isize), (config[1][0] * mult + pos[0] as isize), (config[1][1] * mult + pos[1] as isize)];

            for index in indices {
                if index < 0 {
                    continue 'outer;
                }
            }

            if self.grid[indices[0] as usize][indices[1] as usize][pos[2]] == id
            && self.grid[indices[2] as usize][indices[3] as usize][pos[2]] == id {
                return true;
            }
        }

        false
    }

    fn get_top_left_edge(&self) -> Edge {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                for parity in 0..self.grid[y][x].len() {
                    if self.grid[y][x][parity] != Self::UNFILLED {
                        return [y, x + parity - 1, 1 - parity, 2];
                    }
                }
            }
        }

        panic!("No such top left edge found");
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for (i, row) in self.grid.iter().enumerate() {
            print!("{}", (0..i * 2).map(|_| " ").collect::<String>());
            for square in row {
                print!("\\{}/{}",
                    if square[0] != Self::UNFILLED { square[0].to_string() } else { "-".to_string() },
                    if square[1] != Self::UNFILLED { square[1].to_string() } else { "-".to_string() }
                );
            }
            println!("\\");
        }
    }
}

pub struct Player {
    id: isize,
    max_moves: usize,
    pub pos: Edge,
    pub points: u32,
}

impl Player {
    pub fn new(id: isize, max_moves: usize) -> Self {
        Player { id: id, max_moves: max_moves, pos: [0, 0, 0, 2], points: 0 }
    }

    pub fn make_move(&mut self, board: &mut Board) {
        let mut edge_path = board.get_edge_path_from(self.pos);

        let prev_pos = self.pos; // save previous position for point scoring

        println!("path: {:?}", edge_path);
        self.pos = edge_path[edge_path.len() - 1];

        let edge_path_len = edge_path.len();
        for pos in &mut edge_path[0..std::cmp::min(self.max_moves, edge_path_len)] {
            if board.can_score_point(pos, self.id) {
                self.pos = *pos;
                break;
            }
        }

        board.grid[prev_pos[0]][prev_pos[1]][prev_pos[2]] = self.id;
        if board.can_score_point(&prev_pos, self.id) {
            self.points += 1;
        }
    }

    pub fn needs_repositioning(&self, board: &Board) -> bool {
        println!("{}", board.grid[self.pos[0]][self.pos[1]][self.pos[2]]);
        board.grid[self.pos[0]][self.pos[1]][self.pos[2]] != Board::UNFILLED;
        false
    }

    pub fn reposition(&mut self, board: &Board) {
        self.pos = board.get_top_left_edge();
    }
}

