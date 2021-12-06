//! Wrestled with Iterator and IntoIterator here, but they didn't seem to fit really
use anyhow::ensure;

fn input() -> Vec<usize> {
    include_str!("inputs/2021/6.txt")
        .trim()
        .split(',')
        .map(str::parse::<usize>)
        .collect::<Result<_, _>>()
        .expect("Couldn't parse input")
}

#[derive(Debug, Default, Clone, Copy)]
struct LanternFish {
    timers: [usize; 9],
}

impl LanternFish {
    fn total(&self) -> usize {
        self.timers.into_iter().sum()
    }
    // Can't blanket impl
    fn try_from_iter(iter: impl IntoIterator<Item = usize>) -> anyhow::Result<Self> {
        iter.into_iter()
            .try_fold(LanternFish::default(), |mut acc, el| {
                ensure!(el < 9, "Invalid fish lifetime: {}", el);
                acc.timers[el] += 1;
                anyhow::Ok(acc)
            })
    }
    fn step_simulation(mut self) -> Self {
        let create = self.timers[0];
        self.timers.rotate_left(1);
        self.timers[6] += create; // day 8 will rotate through
        self
    }
}

#[test]
fn part1() {
    let mut fish = LanternFish::try_from_iter(input()).unwrap();
    for _ in 0..80 {
        fish = fish.step_simulation()
    }
    let total = fish.total();
    assert_eq!(total, 390011)
}

#[test]
fn part2() {
    let mut fish = LanternFish::try_from_iter(input()).unwrap();
    for _ in 0..256 {
        fish = fish.step_simulation()
    }
    let total = fish.total();
    assert_eq!(total, 1746710169834)
}
