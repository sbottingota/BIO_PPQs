use std::ops::Mul;

#[derive(PartialEq)]
pub enum IntList {
    Null,
    E,
    O,
    T,
    Combination(Box<IntList>, Box<IntList>)
}

impl Mul<Self> for IntList {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        if self == IntList::Null {
            rhs
        } else {
            IntList::Combination(Box::new(self), Box::new(rhs))
        }
    }
}

impl IntList {
    pub fn index(&self, i: usize) -> usize {
        match self {
            IntList::Null => panic!("Cannot take index of 'IntList::Null'"),
            IntList::E => i * 2,
            IntList::O => i * 2 - 1,
            IntList::T => ((2 * i) as f64).sqrt().round() as usize,
            IntList::Combination(x, y) => y.index(x.index(y.index(i)))
        }
    }
}

