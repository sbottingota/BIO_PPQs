#![allow(dead_code)]

use std::collections::{VecDeque, BTreeSet};

#[derive(Clone, Eq)]
struct State {
    boxes: Vec<char>,
    warehouse: Vec<char>,
    n_operations: usize,
}

impl State {
    fn new(warehouse: &[char]) -> Self {
        Self { warehouse: warehouse.to_vec() , boxes: Vec::new(), n_operations: 0 }
    }

    fn add(&self) -> Option<Self> {
        let mut ret = self.clone();
        if let Some(next_box) = ret.warehouse.pop() {
            ret.boxes.push(next_box);
            ret.n_operations += 1;
            Some(ret)
        } else {
            None
        }
    }

    fn swap(&self) -> Option<Self> {
        if self.boxes.len() >= 2 {
            let mut ret = self.clone();

            ret.boxes.swap(0, 1);
            ret.n_operations += 1;

            Some(ret)
        } else {
            None
        }
    }

    fn rotate(&self) -> Option<Self> {
        if !self.boxes.is_empty() {
            let mut ret = self.clone();

            let first = ret.boxes.remove(0);
            ret.boxes.push(first);
            ret.n_operations += 1;

            Some(ret)
        } else {
            None
        }
    }

    fn next(&self) -> Vec<Self> {
        let mut ret = Vec::new();

        if let Some(added) = self.add() {
            ret.push(added);
        }
        if let Some(swapped) = self.swap() {
            ret.push(swapped);
        }
        if let Some(rotated) = self.rotate() {
            ret.push(rotated);
        }

        ret
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.boxes == other.boxes
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.boxes.cmp(&other.boxes)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn get_n_operations(end: &[char]) -> usize {
    let warehouse = &mut ('A'..='Z').take(end.len()).collect::<Vec<_>>();
    warehouse.reverse();
    let state = State::new(warehouse);
    let mut next_states: VecDeque<State> = VecDeque::from([state.clone()]);
    let mut seen: BTreeSet<State> = BTreeSet::from([state]);

    while let Some(state) = next_states.pop_front() {
        for next in state.next() {
            if end == next.boxes {
                return next.n_operations;
            }

            seen.insert(next.clone());
            next_states.push_back(next);
        }
    }

    panic!("No such path found")
}

fn main() -> std::io::Result<()> {
    let mut display_order = String::new();
    std::io::stdin().read_line(&mut display_order)?;

    println!("{}", get_n_operations(&display_order.trim().chars().collect::<Vec<_>>()));

    Ok(())
}

