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

#[test]
fn test_example() -> anyhow::Result<()> {
    let mut game = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
        .parse::<Game>()?;

    let first = game.next().unwrap();
    assert!(first.0.winner());
    println!("first = {:?}", first.0);
    assert_eq!(first.1, 24);
    assert_eq!(first.0.sum_unmarked(), 188);
    Ok(())
}

#[test]
fn winning() -> anyhow::Result<()> {
    let x = Mark::Marked(0);
    let o = Mark::Unmarked(0);
    let arr = [
        [o, o, o, o, o],
        [x, x, x, x, x],
        [o, o, o, o, o],
        [o, o, o, o, o],
        [o, o, o, o, o],
    ];
    let array = Array2::from_shape_vec((5, 5), arr.into_iter().flatten().collect_vec())?;
    println!("arr = {:?}", array);

    assert!(Board { array }.winner());
    Ok(())
}

fn input() -> Game {
    include_str!("./inputs/2021/4.txt").parse().unwrap()
}

#[test]
fn part1() {
    let (winner, draw) = input().next().unwrap();

    let checksum = winner.sum_unmarked() * draw as usize;
    assert_eq!(checksum, 27027);
}
#[test]
fn part2() {
    let (last_winner, draw) = input().last().unwrap();

    let checksum = last_winner.sum_unmarked() * draw as usize;
    assert_eq!(checksum, 36975);
}
