use anyhow::Context;

extern crate test;

const INPUT: &str = include_str!("./inputs/2021/7.txt");

fn parse(input: &str) -> anyhow::Result<Vec<usize>> {
    input
        .trim()
        .split(',')
        .map(str::parse::<usize>)
        .collect::<Result<_, _>>()
        .context("Invalid input format")
}

fn do_part1(input: &str) -> anyhow::Result<usize> {
    let positions = parse(input)?;
    let max = positions.iter().max().context("Must have one input")?;
    let min = (0..=*max)
        .map(|destination| {
            positions.iter().fold(0, |total, source| {
                let distance: isize = *source as isize - destination as isize;
                total + distance.abs()
            })
        })
        .min()
        .context("Must have one input")?;
    Ok(min as usize)
}
fn do_part2(input: &str) -> anyhow::Result<usize> {
    let positions = parse(input)?;
    let max = positions.iter().max().context("Must have one input")?;
    let min = (0..=*max)
        .map(|destination| {
            positions.iter().fold(0, |total, source| {
                let distance: isize = *source as isize - destination as isize;
                let distance = distance.abs();
                let cost = (distance * (distance + 1)) / 2;
                total + cost
            })
        })
        .min()
        .context("Must have one input")?;
    Ok(min as usize)
}

benchtest::benchtest! {
    part1: do_part1(test::black_box(INPUT)).unwrap() => 323647,
    part2: do_part2(test::black_box(INPUT)).unwrap() => 0
}
