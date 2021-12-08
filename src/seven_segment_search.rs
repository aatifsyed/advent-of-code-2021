use anyhow::{ensure, Context};
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};

extern crate test;

const INPUT: &str = include_str!("./inputs/2021/8.txt");

struct Observation {
    inputs: Vec<HashSet<char>>,
    outputs: Vec<HashSet<char>>,
}

impl Observation {
    fn decode(self) -> anyhow::Result<usize> {
        let mut inputs = self.inputs;
        let one = remove(&mut inputs, |set| set.len() == 2)?;
        let four = remove(&mut inputs, |set| set.len() == 4)?;
        let seven = remove(&mut inputs, |set| set.len() == 3)?;
        let eight = remove(&mut inputs, |set| set.len() == 7)?;
        let three = remove(&mut inputs, |set| {
            set.len() == 5 && set.intersection(&seven).count() == 3
        })?;
        let nine = remove(&mut inputs, |set| {
            set.len() == 6 && set.intersection(&seven).count() == 3
        })?;
        let zero = remove(&mut inputs, |set| {
            set.len() == 6 && set.intersection(&one).count() == 1
        })?;
        let two = remove(&mut inputs, |set| {
            set.len() == 5 && set.intersection(&four).count() == 2
        })?;
        let five = remove(&mut inputs, |set| {
            set.len() == 5 && set.intersection(&four).count() == 3
        })?;
        let six = remove(&mut inputs, |set| {
            set.len() == 6 && set.intersection(&one).count() == 0
        })?;
        let value = self
            .outputs
            .into_iter()
            .map(|set| {
                if set == zero {
                    '0'
                } else if set == one {
                    '1'
                } else if set == two {
                    '2'
                } else if set == three {
                    '3'
                } else if set == four {
                    '4'
                } else if set == five {
                    '5'
                } else if set == six {
                    '6'
                } else if set == seven {
                    '7'
                } else if set == eight {
                    '8'
                } else if set == nine {
                    '9'
                } else {
                    '!'
                }
            })
            .collect::<String>();
        Ok(value.parse()?)
    }
}

fn remove<T>(v: &mut Vec<T>, mut predicate: impl FnMut(&T) -> bool) -> anyhow::Result<T> {
    let position = v
        .iter()
        .find_position(|t| predicate(*t))
        .context("Not found")?
        .0;
    Ok(v.swap_remove(position))
}

impl FromStr for Observation {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ensure!(
            value.chars().all(|c| matches!(c, 'a'..='g' | ' ' | '|')),
            "Disallowed character in line"
        );
        let split = value.split('|').collect_vec();
        ensure!(split.len() == 2, "Must be two observations");
        let inputs = split[0]
            .split_whitespace()
            .map(|s| HashSet::from_iter(s.chars()))
            .collect_vec();
        ensure!(inputs.len() == 10, "Must be 10 inputs");
        let outputs = split[1]
            .split_whitespace()
            .map(|s| HashSet::from_iter(s.chars()))
            .collect_vec();
        ensure!(outputs.len() == 4, "Must be 10 outputs");
        Ok(Self { inputs, outputs })
    }
}

fn parse(input: &str) -> anyhow::Result<Vec<Observation>> {
    input.lines().map(str::parse).collect::<Result<_, _>>()
}

fn do_part1(input: &str) -> anyhow::Result<usize> {
    let observations = parse(input)?;
    let c = observations
        .into_iter()
        .flat_map(|o| o.outputs)
        .filter(|o| match o.len() {
            2 | 4 | 3 | 7 => true,
            _ => false,
        })
        .count();
    Ok(c)
}
fn do_part2(input: &str) -> anyhow::Result<usize> {
    let res = parse(input)?
        .into_iter()
        .map(Observation::decode)
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();

    Ok(res)
}

benchtest::benchtest! {
    part1: do_part1(test::black_box(INPUT)).unwrap() => 412,
    part2: do_part2(test::black_box(INPUT)).unwrap() => 0
}
