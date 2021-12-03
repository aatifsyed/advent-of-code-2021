/// There must be a better way...
use array2d::Array2D;
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::Hash,
};

fn input() -> Array2D<char> {
    let v = include_str!("inputs/2021/3.txt")
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let arr = Array2D::from_rows(&v);
    arr
}

fn counts<T: Hash + Eq>(it: impl IntoIterator<Item = T>) -> HashMap<T, usize> {
    it.into_iter().fold(HashMap::new(), |mut counts, el| {
        counts
            .entry(el)
            .and_modify(|count| *count += 1)
            .or_default();
        counts
    })
}

fn most_common<T: Hash + Eq>(it: impl IntoIterator<Item = T>) -> Option<T> {
    counts(it)
        .into_iter()
        .reduce(|winner, challenger| match winner.1 > challenger.1 {
            true => winner,
            false => challenger,
        })
        .map(|(t, _count)| t)
}

fn least_common<T: Hash + Eq>(it: impl IntoIterator<Item = T>) -> Option<T> {
    counts(it)
        .into_iter()
        .reduce(|winner, challenger| match winner.1 < challenger.1 {
            true => winner,
            false => challenger,
        })
        .map(|(t, _count)| t)
}

fn gamma_rate(input: &Array2D<char>) -> usize {
    let s = input
        .columns_iter()
        .map(|row| most_common(row).expect("Non-empty"))
        .collect::<String>();
    usize::from_str_radix(&s, 2).expect("Only 1s and 0s")
}

fn epsilon_rate(input: &Array2D<char>) -> usize {
    let s = input
        .columns_iter()
        .map(|row| least_common(row).expect("Non-empty"))
        .collect::<String>();
    usize::from_str_radix(&s, 2).expect("Binary")
}

#[test]
fn part1() {
    let input = &input();
    let epsilon = epsilon_rate(input);
    let gamma = gamma_rate(input);
    assert_eq!(epsilon * gamma, 0)
}

// Well that was a lot of wasted work!
// Let's just go imperative
fn common_rating(input: &Array2D<char>, preferrer: impl Fn(Ordering) -> char) -> usize {
    let mut possible = input
        .rows_iter()
        .map(Iterator::collect::<String>)
        .collect::<HashSet<_>>();

    for i in 0..input.num_columns() {
        if possible.len() == 1 {
            break;
        }

        let counts = counts(
            possible
                .iter()
                .map(|s| s.chars().nth(i).expect("Not indexing beyond num_columns")),
        );

        let num_zeroes = counts.get(&'0').unwrap_or(&0);
        let num_ones = counts.get(&'1').unwrap_or(&0);

        // Ew
        let preferred = preferrer(num_zeroes.cmp(num_ones));

        possible.retain(|s| {
            let c = s.chars().nth(i).expect("Not indexing beyond num_columns");
            c == preferred
        });
    }
    assert_eq!(possible.len(), 1);
    let rating = possible.drain().next().expect("Non-empty");
    usize::from_str_radix(&rating, 2).expect("Binary")
}

fn oxygen_generator_rating(input: &Array2D<char>) -> usize {
    common_rating(input, |o| match o {
        Ordering::Less => '1',
        Ordering::Equal => '1',
        Ordering::Greater => '0',
    })
}

fn co2_scrubber_rating(input: &Array2D<char>) -> usize {
    common_rating(input, |o| match o {
        Ordering::Less => '0',
        Ordering::Equal => '0',
        Ordering::Greater => '1',
    })
}

#[test]
fn example() {
    let s = "\
        00100\n\
        11110\n\
        10110\n\
        10111\n\
        10101\n\
        01111\n\
        00111\n\
        11100\n\
        10000\n\
        11001\n\
        00010\n\
        01010"
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let input = Array2D::from_rows(&s);
    let rating = oxygen_generator_rating(&input);
    assert_eq!(rating, 23);

    let rating = co2_scrubber_rating(&input);
    assert_eq!(rating, 10);
}

#[test]
fn part2() {
    let input = input();
    let o2_rating = oxygen_generator_rating(&input);
    let co2_rating = co2_scrubber_rating(&input);
    assert_eq!(o2_rating * co2_rating, 0)
}
