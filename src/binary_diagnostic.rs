/// There must be a better way...
/// ## Next time...
/// - Try a derivable enum BitCount { Zero(usize) }...
///   As a kind of perfect hashmap, maybe with a nice entry api!
/// - Use fallible functions everywhere
/// ## Missed tricks
/// - gamma = bit-flipped epsilon
use anyhow::anyhow;
use array2d::Array2D;
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Bit {
    Zero,
    One,
}

impl TryFrom<char> for Bit {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            _ => Err(anyhow!("Invalid bit: {}", value)),
        }
    }
}

impl From<&Bit> for char {
    fn from(b: &Bit) -> Self {
        match b {
            Bit::Zero => '0',
            Bit::One => '1',
        }
    }
}

fn input() -> Array2D<Bit> {
    make_array(include_str!("inputs/2021/3.txt"))
}

fn make_array(s: &str) -> Array2D<Bit> {
    let v = s
        .lines()
        .map(|line| line.chars().map(Bit::try_from).try_collect())
        .try_collect::<_, Vec<_>, _>()
        .expect("Binary input");
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

fn part1_generic<'a>(
    input: &'a Array2D<Bit>,
    mut select_bit_from_column: impl FnMut(Vec<&Bit>) -> &Bit, // Can't use fn(impl Iterator)...?
) -> usize {
    let s = input
        .columns_iter()
        .map(|column| select_bit_from_column(column.collect_vec()))
        .map(char::from)
        .collect::<String>();
    usize::from_str_radix(&dbg!(s), 2).expect("Binary")
}

fn gamma_rate(input: &Array2D<Bit>) -> usize {
    part1_generic(input, |v| most_common(v).expect("Non-empty"))
}

fn epsilon_rate(input: &Array2D<Bit>) -> usize {
    part1_generic(input, |v| least_common(v).expect("Non-empty"))
}

#[test]
fn part1() {
    let input = &input();
    let epsilon = epsilon_rate(input);
    let gamma = gamma_rate(input);
    assert_eq!(epsilon * gamma, 4139586)
}

// Well that was a lot of wasted work!
// Let's just go imperative
fn part2_generic(input: &Array2D<Bit>, preferrer: impl Fn(HashMap<Bit, usize>) -> Bit) -> usize {
    let mut possible = input
        .rows_iter()
        .map(|row| row.map(Clone::clone).collect_vec())
        .collect::<HashSet<_>>();

    for i in 0..input.num_columns() {
        if possible.len() == 1 {
            break;
        }

        let counts = counts(possible.iter().map(|s| s[i]));
        let preferred = preferrer(counts);
        possible.retain(|s| s[i] == preferred);
    }
    assert_eq!(possible.len(), 1);
    let rating = possible
        .drain()
        .next()
        .expect("Non-empty")
        .iter()
        .map(char::from)
        .collect::<String>();
    usize::from_str_radix(&rating, 2).expect("Binary")
}

fn oxygen_generator_rating(input: &Array2D<Bit>) -> usize {
    part2_generic(input, |counts| {
        let num_zeroes = counts.get(&Bit::Zero).unwrap_or(&0);
        let num_ones = counts.get(&Bit::One).unwrap_or(&0);
        match num_zeroes.cmp(num_ones) {
            Ordering::Less => Bit::One,
            Ordering::Equal => Bit::One,
            Ordering::Greater => Bit::Zero,
        }
    })
}

fn co2_scrubber_rating(input: &Array2D<Bit>) -> usize {
    part2_generic(input, |counts| {
        let num_zeroes = counts.get(&Bit::Zero).unwrap_or(&0);
        let num_ones = counts.get(&Bit::One).unwrap_or(&0);
        match num_zeroes.cmp(num_ones) {
            Ordering::Less => Bit::Zero,
            Ordering::Equal => Bit::Zero,
            Ordering::Greater => Bit::One,
        }
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
        01010";
    let input = make_array(s);
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
    assert_eq!(o2_rating * co2_rating, 1800151)
}
