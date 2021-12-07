//! Wrestled with Iterator and IntoIterator here, but they didn't seem to fit really
use anyhow::ensure;

use anyhow::Context;

extern crate test;

const INPUT: &str = include_str!("./inputs/2021/6.txt");

fn do_part1(input: &str) -> anyhow::Result<usize> {
    let mut fish = LanternFish::try_from_iter(parse(input)?).unwrap();
    for _ in 0..80 {
        fish = fish.step_simulation()
    }
    Ok(fish.total())
}
fn do_part2(input: &str) -> anyhow::Result<usize> {
    let mut fish = LanternFish::try_from_iter(parse(input)?).unwrap();
    for _ in 0..256 {
        fish = fish.step_simulation()
    }
    Ok(fish.total())
}

benchtest::benchtest! {
    part1: do_part1(test::black_box(INPUT)).unwrap() => 390011,
    part2: do_part2(test::black_box(INPUT)).unwrap() => 1746710169834
}

fn parse(input: &str) -> anyhow::Result<Vec<usize>> {
    input
        .trim()
        .split(',')
        .map(str::parse::<usize>)
        .collect::<Result<_, _>>()
        .context("Invalid input format")
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
