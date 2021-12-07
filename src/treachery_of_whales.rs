use anyhow::Context;
use num::Num;

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
    let min_cost = (0..=*max)
        .map(|destination| {
            positions.iter().fold(0, |total, source| {
                let distance: isize = *source as isize - destination as isize;
                total + distance.abs() // abs_diff actually takes twice as long
            })
        })
        .min()
        .context("Must have one input")?;
    Ok(min_cost as usize)
}

fn optimized(input: &str) -> anyhow::Result<(usize, usize)> {
    let mut positions = parse(input)?;
    let mean = mean(&positions); // position part 1
    let median = *median(&mut positions).context("Empty")?; // position part 2

    let mut part1 = 0;
    let mut part2 = 0;

    for position in positions {
        part1 += position.abs_diff(median);
        part2 += gaussian_sum(position.abs_diff(mean));
    }

    Ok((part1, part2))
}

fn gaussian_sum(to: usize) -> usize {
    // n(n+1)
    //   2
    (to * (to + 1)) / 2
}

fn mean<T: Num + Copy>(of: &[T]) -> T {
    let (total, length) = of
        .iter()
        .fold((T::zero(), T::zero()), |(total, length), el| {
            (total + *el, length + T::one())
        });
    total / length
}

fn median<T: Num + Ord>(of: &mut [T]) -> Option<&T> {
    of.sort_unstable();
    let halfway = of.len() / 2;
    of.get(halfway)
}

fn do_part2(input: &str) -> anyhow::Result<usize> {
    let positions = parse(input)?;
    let max = positions.iter().max().context("Must have one input")?;
    let min_cost = (0..=*max)
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
    Ok(min_cost as usize)
}

benchtest::benchtest! {
    part1_brute: do_part1(test::black_box(INPUT)).unwrap() => 323647,

    part2_brute: do_part2(test::black_box(INPUT)).unwrap() => 87640209,
    both_optimized: optimized(test::black_box(INPUT)).unwrap() => (323647, 87640209)
}
