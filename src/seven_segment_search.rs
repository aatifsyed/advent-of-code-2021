use anyhow::{anyhow, ensure, Context};
use itertools::Itertools;

use crate::utils::CountOccurences;

extern crate test;

const INPUT: &str = include_str!("./inputs/2021/8.txt");

struct Observation<'a> {
    inputs: [&'a str; 10],
    outputs: [&'a str; 4],
}

impl<'a> TryFrom<&'a str> for Observation<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        ensure!(
            value.chars().all(|c| matches!(c, 'a'..='g' | ' ' | '|')),
            "Disallowed character in line"
        );
        let mut split = value.split('|').collect_vec();
        ensure!(split.len() == 2, "Must be two observations");
        let inputs = split[0].split_whitespace().collect_vec();
        let outputs = split[1].split_whitespace().collect_vec();
        Ok(Self {
            inputs: inputs
                .try_into()
                .map_err(|_| anyhow!("Wrong number of inputs"))?,
            outputs: outputs
                .try_into()
                .map_err(|_| anyhow!("Wrong number of outputs"))?,
        })
    }
}

fn do_part1(input: &str) -> anyhow::Result<usize> {
    let observations = input
        .lines()
        .map(Observation::try_from)
        .collect::<Result<Vec<_>, _>>()?;
    let counts = observations
        .into_iter()
        .map(|observation| observation.outputs.map(str::len))
        .flatten()
        .count_occurences();
    let ones = *counts.get(&2).unwrap_or(&0);
    let fours = *counts.get(&4).unwrap_or(&0);
    let sevens = *counts.get(&3).unwrap_or(&0);
    let eights = *counts.get(&7).unwrap_or(&0);
    Ok(ones + fours + sevens + eights)
}
fn do_part2(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

benchtest::benchtest! {
    part1: do_part1(test::black_box(INPUT)).unwrap() => 0,
    part2: do_part2(test::black_box(INPUT)).unwrap() => 0
}
