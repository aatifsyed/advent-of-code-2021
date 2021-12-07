/// # Next time
/// - Use a set for each row, column, so you can just test for membership
use anyhow::Context;
use itertools::{zip, Itertools};
use ndarray::Array2;
use std::{
    collections::VecDeque,
    hash::Hash,
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(Debug)]
struct Game {
    current_draw: u8,
    future_draws: VecDeque<u8>,
    boards: Vec<Board>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut future_draws = s
            .lines()
            .next()
            .context("No first line")?
            .split(',')
            .map(str::parse)
            .collect::<Result<VecDeque<_>, _>>()
            .context("Not CSV")?;

        let mut boards = s
            .split("\n\n")
            .skip(1) // First line is CSV
            .map(Board::from_str)
            .collect::<Result<Vec<_>, _>>()
            .context("Boards")?;

        // Do first round
        let current_draw = future_draws.pop_front().context("Must have one draw")?;
        for board in boards.iter_mut() {
            board.mark_at(current_draw)
        }

        Ok(Game {
            current_draw,
            future_draws,
            boards,
        })
    }
}

impl Iterator for Game {
    type Item = (Board, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos) = self.boards.iter().position(Board::winner) {
            println!("exiting winner at {}", pos);
            return Some((self.boards.swap_remove(pos), self.current_draw));
        }
        while let Some(draw) = self.future_draws.pop_front() {
            println!("draw = {}", draw);
            self.current_draw = draw;
            for board in self.boards.iter_mut() {
                board.mark_at(self.current_draw)
            }
            if let Some(pos) = self.boards.iter().position(Board::winner) {
                println!("new winner at = {}", pos);
                return Some((self.boards.swap_remove(pos), self.current_draw));
            }
        }
        None
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Board {
    array: Array2<Mark<u8>>,
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lin = s
            .split_whitespace()
            .map(str::parse)
            .map_ok(Mark::unmarked)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Board {
            array: Array2::from_shape_vec((5, 5), lin)?,
        })
    }
}

impl Board {
    fn winner(&self) -> bool {
        let winning_column = self
            .array
            .columns()
            .into_iter()
            .map(|column| column.into_iter().all(Mark::is_marked))
            .any(|winning_column| winning_column == true);

        let winning_row = self
            .array
            .rows()
            .into_iter()
            .map(|row| row.into_iter().all(Mark::is_marked))
            .any(|winning_row| winning_row == true);

        // diagonals don't count 🤦
        // ^-_
        let _descending = (0..5)
            .map(|co| self.array.get((co, co)).unwrap())
            .all(Mark::is_marked);

        // _-^
        let _ascending = zip(0..5, (0..5).rev())
            .map(|(row, column)| self.array.get((row, column)).unwrap())
            .all(Mark::is_marked);

        winning_column || winning_row
    }

    fn sum_unmarked(self) -> usize {
        self.array
            .into_raw_vec()
            .into_iter()
            .filter_map(|mark| match mark {
                Mark::Marked(_) => None,
                Mark::Unmarked(t) => Some(t as usize),
            })
            .sum()
    }

    fn mark_at(&mut self, value: u8) {
        for element in self.array.iter_mut() {
            if **element == value {
                element.mark()
            }
        }
    }
}

#[test]
fn zippin() {
    let v = zip(0..5, (0..5).rev()).collect_vec();
    println!("v = {:?}", v);
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Mark<T> {
    Marked(T),
    Unmarked(T),
}
impl<T> Mark<T> {
    fn unmarked(t: T) -> Self {
        Mark::Unmarked(t)
    }
    fn into_inner(self) -> T {
        match self {
            Mark::Marked(t) => t,
            Mark::Unmarked(t) => t,
        }
    }

    /// Returns `true` if the mark is [`Marked`].
    ///
    /// [`Marked`]: Mark::Marked
    fn is_marked(&self) -> bool {
        matches!(self, Self::Marked(..))
    }
}

impl<T: Clone> Mark<T> {
    fn mark(&mut self) {
        let t = self.clone().into_inner();
        *self = Mark::Marked(t)
    }
}

impl<T> Deref for Mark<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Mark::Marked(t) => t,
            Mark::Unmarked(t) => t,
        }
    }
}

impl<T> DerefMut for Mark<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Mark::Marked(t) => t,
            Mark::Unmarked(t) => t,
        }
    }
}

extern crate test;

const INPUT: &str = include_str!("./inputs/2021/4.txt");

fn do_part1(input: &str) -> anyhow::Result<usize> {
    let (winner, draw) = input
        .parse::<Game>()?
        .next()
        .context("Game has no winner")?;
    let checksum = winner.sum_unmarked() * draw as usize;
    Ok(checksum)
}
fn do_part2(input: &str) -> anyhow::Result<usize> {
    let (winner, draw) = input
        .parse::<Game>()?
        .last()
        .context("Game has no winner")?;
    let checksum = winner.sum_unmarked() * draw as usize;
    Ok(checksum)
}

benchtest::benchtest! {
    part1: do_part1(test::black_box(INPUT)).unwrap() => 27027,
    part2: do_part2(test::black_box(INPUT)).unwrap() => 36975
}
