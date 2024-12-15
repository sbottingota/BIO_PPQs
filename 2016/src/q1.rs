struct Fraction {
    num: usize,
    denom: usize,

    l: usize,
    m: usize,
    
    r: usize,
    s: usize,
}

impl Fraction {
    fn new() -> Fraction {
        Fraction { num: 1, denom: 1, l: 1, m: 0, r: 0, s: 1 }
    }

    fn left(&mut self) {
        self.l = self.num;
        self.m = self.denom;

        self.num = self.l + self.r;
        self.denom = self.m + self.s;
    }

    fn right(&mut self) {
        self.r = self.num;
        self.s = self.denom;

        self.num = self.l + self.r;
        self.denom = self.m + self.s;
    }

    fn get_frac(&self) -> (usize, usize) {
        (self.num, self.denom)
    }
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);

    let mut frac = Fraction::new();    

    for c in buf.chars() {
        if c == 'L' {
            frac.left();
        } else if c == 'R' {
            frac.right();
        }
    }

    let val = frac.get_frac();
    println!("{} / {}", val.0, val.1);
}

