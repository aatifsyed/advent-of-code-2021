use anyhow::Context;
use itertools::Itertools;

extern crate test;

const INPUT: &str = include_str!("./inputs/2021/1.txt");

fn parse(input: &str) -> anyhow::Result<Vec<usize>> {
    input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .context("Incorrect input format")
}

fn do_part1(input: &str) -> anyhow::Result<usize> {
    let num_downward_steps = parse(input)?
        .iter()
        .tuple_windows()
        .filter(|(near, far)| far > near)
        .count();
    Ok(num_downward_steps)
}
fn do_part2(input: &str) -> anyhow::Result<usize> {
    let num_downward_steps_grouped = parse(input)?
        .windows(3)
        .tuple_windows()
        .filter(|(near_group, far_group)| far_group.iter().sum::<usize>() > near_group.iter().sum())
        .count();
    Ok(num_downward_steps_grouped)
}

benchtest::benchtest! {
    part1: do_part1(test::black_box(INPUT)).unwrap() => 1316,
    part2: do_part2(test::black_box(INPUT)).unwrap() => 1344
}
