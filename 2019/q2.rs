use std::collections::VecDeque;

const N_DIRECTIONS: usize = 4;

type Coord = (isize, isize);

#[repr(usize)]
#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Direction {
    Forward = 0,
    Right = 1,
    Back = 2,
    Left = 3,
}

impl Direction {
    fn right(&self) -> Direction {
        Self::from_int((*self as usize + 1) % N_DIRECTIONS)
    }

    fn left(&self) -> Direction {
        Self::from_int((*self as usize + 3) % N_DIRECTIONS)
    }

    fn from_int(i: usize) -> Direction {
        unsafe { std::mem::transmute(i) }
    }

    fn coord(&self) -> Coord {
        match *self as usize {
            0 => (0, 1),
            1 => (1, 0),
            2 => (0, -1),
            3 => (-1, 0),
            _ => panic!("Invalid direction")
        }
    }

    fn moved(&self, coord: &Coord) -> Coord {
        (self.coord().0 + coord.0, self.coord().1 + coord.1)
    }
}

pub struct Explorer {
    pub pos: Coord,
    facing: Direction,
    trail: VecDeque<Coord>,
    trail_length: usize
}

impl Explorer {
    pub fn new(trail_length: usize) -> Self {
        Explorer { pos: (0, 0), facing: Direction::Forward, trail: VecDeque::new(), trail_length: trail_length }
    }

    pub fn step(&mut self, prefered_step: Direction) -> Result<(), ()> {
        self.trail.push_back(self.pos);

        if prefered_step == Direction::Right {
            self.facing = self.facing.right();
        } else if prefered_step == Direction::Left {
            self.facing = self.facing.left();
        }

        let mut i = 0_usize;
        while self.trail.contains(&self.facing.moved(&self.pos)) && i < N_DIRECTIONS {
            self.facing = self.facing.right();
            i += 1;
        }

        // no possible moves
        if i == N_DIRECTIONS {
            return Err(());
        }

        self.pos = self.facing.moved(&self.pos);

        if self.trail.len() > self.trail_length {
            self.trail.pop_front();
        }

        Ok(())
    }
}

