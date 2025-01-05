use std::collections::HashMap;

#[derive(Debug)]
struct Grid {
    people: HashMap<(isize, isize), usize>,
}

impl Grid {
    const MIGRATION_THRESHOLD: usize = 4;

    fn new() -> Self {
        Self { people: HashMap::new(),
        }
    }

    fn add_person(&mut self, pos: (isize, isize)) {
        if self.people.contains_key(&pos) {
            *self.people.get_mut(&pos).unwrap() += 1;
        } else {
            self.people.insert(pos, 1);
        }
    }

    fn migrate_if_needed(&mut self) {
        for pos in self.people.clone().keys() {
            if self.people[pos] >= Self::MIGRATION_THRESHOLD {
                self.migrate_square(*pos);
            }
        }
    }

    fn migrate_square(&mut self, pos: (isize, isize)) {
        *self.people.get_mut(&pos).unwrap() -= Self::MIGRATION_THRESHOLD;

        self.add_person((pos.0 - 1, pos.1));
        self.add_person((pos.0, pos.1 - 1));
        self.add_person((pos.0 + 1, pos.1));
        self.add_person((pos.0, pos.1 + 1));
    }

    fn pos_from_square_id(id: usize) -> (isize, isize) {
        (id as isize / 5, id as isize % 5)
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..5 {
            writeln!(f)?;
            for y in 0..5 {
                if let Some(n_people) = self.people.get(&(x, y)) {
                    write!(f, "{} ", n_people)?;
                } else {
                    write!(f, "0 ")?;
                }
            }
        }

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut buf)?;
    let &[start, _sequence_len, n_steps] = &buf
        .split_whitespace()
        .map(|substr| substr.parse::<usize>().unwrap())
        .collect::<Vec<_>>()[..]
        else { panic!("Invalid input"); };

    buf.clear();
    stdin.read_line(&mut buf)?;
    let sequence: Vec<usize> = buf
        .split_whitespace()
        .map(|substr| substr.parse().unwrap())
        .collect();


    let mut grid = Grid::new();

    let mut pos_id = start - 1;

    for i in 0..n_steps {
        grid.add_person(Grid::pos_from_square_id(pos_id));
        grid.migrate_if_needed();

        pos_id = (pos_id + sequence[i % sequence.len()]) % 25;
    }

    println!("{}", grid);

    Ok(())
}
