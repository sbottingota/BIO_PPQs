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

    const EDGE_CONFIGURATIONS: [[[([isize; 3], [isize; 4]); 5]; 3]; 2] = [
        [
            [
                ([0_isize, -1_isize, 0_isize], [0_isize, -1_isize, 1_isize, 0_isize]),
                ([1_isize, -1_isize, 1_isize], [0_isize, -1_isize, 0_isize, 1_isize]),
                ([1_isize, 0_isize, 0_isize], [1_isize, -1_isize, 1_isize, 2_isize]),
                ([1_isize, 0_isize, 1_isize], [1_isize, 0_isize, 0_isize, 2_isize]),
                ([0_isize, 0_isize, 0_isize], [1_isize, 0_isize, 1_isize, 1_isize])
            ],
            [
                ([1_isize, 1_isize, 0_isize], [1_isize, 0_isize, 1_isize, 2_isize]),
                ([1_isize, 1_isize, 1_isize], [1_isize, 1_isize, 0_isize, 2_isize]),
                ([0_isize, 1_isize, 0_isize], [1_isize, 1_isize, 1_isize, 1_isize]),
                ([0_isize, 0_isize, 1_isize], [0_isize, 1_isize, 0_isize, 0_isize]),
                ([0_isize, 0_isize, 0_isize], [0_isize, 0_isize, 1_isize, 2_isize])
            ],
            [
                ([-1_isize, 0_isize, 0_isize], [0_isize, 0_isize, 1_isize, 1_isize]),
                ([-1_isize, -1_isize, 1_isize], [-1_isize, 0_isize, 0_isize, 0_isize]),
                ([-1_isize, -1_isize, 0_isize], [-1_isize, -1_isize, 1_isize, 0_isize]),
                ([0_isize, -1_isize, 1_isize], [-1_isize, -1_isize, 0_isize, 1_isize]),
                ([0_isize, 0_isize, 0_isize], [0_isize, -1_isize, 1_isize, 2_isize])
            ]
        ],
        [
            [
                ([1_isize, 0_isize, 1_isize], [0_isize, 0_isize, 0_isize, 1_isize]),
                ([1_isize, 1_isize, 0_isize], [1_isize, 0_isize, 1_isize, 2_isize]),
                ([1_isize, 1_isize, 1_isize], [1_isize, 1_isize, 0_isize, 2_isize]),
                ([0_isize, 1_isize, 0_isize], [1_isize, 1_isize, 1_isize, 1_isize]),
                ([0_isize, 0_isize, 1_isize], [0_isize, 1_isize, 0_isize, 2_isize])
            ],
            [
                ([-1_isize, -1_isize, 1_isize], [-1_isize, 0_isize, 0_isize, 0_isize]),
                ([-1_isize, -1_isize, 0_isize], [-1_isize, -1_isize, 1_isize, 0_isize]),
                ([0_isize, 1_isize, 1_isize], [-1_isize, -1_isize, 0_isize, 1_isize]),
                ([0_isize, 0_isize, 0_isize], [0_isize, 1_isize, 1_isize, 0_isize]),
                ([0_isize, 0_isize, 1_isize], [0_isize, 0_isize, 0_isize, 2_isize])
            ],
            [
                ([0_isize, 1_isize, 1_isize], [0_isize, 1_isize, 0_isize, 2_isize]),
                ([-1_isize, 1_isize, 0_isize], [0_isize, 1_isize, 1_isize, 1_isize]),
                ([-1_isize, 0_isize, 1_isize], [-1_isize, 1_isize, 0_isize, 0_isize]),
                ([-1_isize, 0_isize, 0_isize], [-1_isize, 0_isize, 1_isize, 0_isize]),
                ([0_isize, 0_isize, 1_isize], [-1_isize, 0_isize, 0_isize, 1_isize])
            ]
]
    ];

    pub fn new() -> Self {
        let mut board = Board { grid: VecDeque::new() };
        board.grid.push_front(VecDeque::new());
        board.grid[0].push_front([Self::UNFILLED, 0]);
        for _ in 0..2 {
            board.enlarge();
        }
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
        let configs = Self::EDGE_CONFIGURATIONS[prev[2]][prev[3]];

        'outer: for config in configs {

            let indices = [(prev[0] as isize + config.0[0]), (prev[1] as isize + config.0[1])];
            for index in indices {
                if index < 0 {
                    continue 'outer;
                }
            }

            if self.grid[indices[0] as usize][indices[1] as usize][prev[2]] == Self::UNFILLED {
                return [(prev[0] as isize + config.1[0]) as usize, (prev[1] as isize + config.1[1]) as usize, config.1[2] as usize, config.1[3] as usize];
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
                return vertices;
            }

            vertices.push(current);

        }
    }

    fn can_score_point(&self, pos: Edge, id: isize) -> bool {
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
                        return [y, x, parity, 0];
                    }
                }
            }
        }

        panic!("No such top left edge found");
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
        let edge_path = board.get_edge_path_from(self.pos);
        board.grid[self.pos[0]][self.pos[1]][self.pos[2]] = self.id;
        if board.can_score_point(self.pos, self.id) {
            self.points += 1;
        }

        println!("{:?}", edge_path);
        for i in 0..self.max_moves {
            if board.can_score_point(edge_path[i % edge_path.len()], self.id) {
                self.pos = edge_path[i];
                break;
            }
        }
    }

    pub fn needs_repositioning(&self, board: &Board) -> bool {
        board.grid[self.pos[0]][self.pos[1]][self.pos[2]] != Board::UNFILLED
    }

    pub fn reposition(&mut self, board: &Board) {
        self.pos = board.get_top_left_edge();
    }
}

