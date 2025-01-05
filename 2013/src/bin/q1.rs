struct Clock {
    hours: usize,
    minutes: usize,
    n_minutes_fast: usize,
}

impl Clock {
    const MINUTES_IN_HOUR: usize = 60;
    const HOURS_IN_DAY: usize = 24;

    fn new(n_minutes_fast: usize) -> Self {
        Self { hours: 0, minutes: 0, n_minutes_fast }
    }

    fn increment(&mut self) {
        self.hours += 1;

        self.minutes += self.n_minutes_fast;
        while self.minutes >= Self::MINUTES_IN_HOUR {
            self.minutes -= Self::MINUTES_IN_HOUR;
            self.hours += 1;
        }

        self.hours %= Self::HOURS_IN_DAY;
    }
}

fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    let &[clock1_n_min_fast, clock2_n_min_fast] = &buf.split_whitespace().map(|substr| substr.parse::<usize>().unwrap()).collect::<Vec<_>>()[..] else { panic!("Invalid input") };

    let mut clock1 = Clock::new(clock1_n_min_fast);
    let mut clock2 = Clock::new(clock2_n_min_fast);

    loop {
        clock1.increment();
        clock2.increment();

        if clock1.hours == clock2.hours && clock1.minutes == clock2.minutes {
            break;
        }
    }

    println!("{:02}:{:02}", clock1.hours, clock1.minutes);

    Ok(())
}

