use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use anyhow::{anyhow, Context};
use itertools::Itertools;

#[derive(Debug)]
struct Game {
    draw: Vec<u8>,
    boards: Vec<Board>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let draw = s
            .lines()
            .next()
            .context("No first line")?
            .split(',')
            .map(str::parse)
            .try_collect()
            .context("Not CSV")?;

        let boards = s
            .split("\n\n")
            .skip(1) // First line is CSV
            .map(Board::from_str)
            .try_collect()
            .context("Boards")?;

        Ok(Game { draw, boards })
    }
}

#[derive(Debug)]
struct Board {
    array: [[Mark<u8>; 5]; 5],
}

impl Board {
    fn wins(&self) -> bool {
        todo!()
    }
    fn rows(&self) -> impl IntoIterator<Item = &[Mark<u8>; 5]> {
        self.array.iter()
    }
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let array = s
            .lines()
            .map(|line| {
                let array = line
                    .split_whitespace()
                    .map(str::parse)
                    .map_ok(Mark::unmarked)
                    .collect::<Result<Vec<_>, _>>()?
                    .try_into()
                    .map_err(|_| anyhow!("Vec has wrong length"))?;
                anyhow::Ok(array)
            })
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| anyhow!("Vec has wrong length"))?;
        Ok(Board { array })
    }
}

#[derive(Debug, Clone, Copy)]
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
fn test_parse_example() -> anyhow::Result<()> {
    let game = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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

    println!("game = {:?}", game);
    Ok(())
}
