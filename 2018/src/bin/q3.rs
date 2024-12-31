use std::cmp::{min, max, Ordering};
use std::collections::{BTreeSet, VecDeque};

#[derive(Clone, Debug, Eq)]
struct State {
    number: Vec<usize>,
    distance: usize,
}

impl State {
    fn new(number: Vec<usize>) -> Self {
        Self { number, distance: 0 }
    }

    fn get_all_transformations(&self) -> Vec<Self> {
        let mut ret = Vec::new();

        for i in 0..self.number.len() - 1 {
            if i > 0 && min(self.number[i], self.number[i + 1]) < self.number[i - 1] && max(self.number[i], self.number[i + 1]) > self.number[i - 1]
                || i < self.number.len() - 2 && min(self.number[i], self.number[i + 1]) < self.number[i + 2] && max(self.number[i], self.number[i + 1]) > self.number[i + 2] {
                let mut new = self.clone();
                new.number.swap(i, i + 1);
                new.distance += 1;
                ret.push(new);
            }
        }

        ret
    }

    fn max_dist(&self) -> usize {
        let mut to_visit: VecDeque<Self> = VecDeque::from([self.clone()]);
        let mut seen: BTreeSet<Self> = BTreeSet::from([self.clone()]);

        while let Some(state) = to_visit.pop_front() {
            for next in state.get_all_transformations() {
                if !seen.contains(&next) {
                    seen.insert(next.clone());
                    to_visit.push_back(next);
                }
            }
        }

        seen.into_iter().max_by(|state1, state2| state1.distance.cmp(&state2.distance)).unwrap().distance
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    let stdin = std::io::stdin();

    // discard first line of input
    stdin.read_line(&mut buf)?;
    buf.clear();

    stdin.read_line(&mut buf)?;

    let state = State::new(buf.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect());

    println!("{}", state.max_dist());

    Ok(())
}

