use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use anyhow::{anyhow, Context};
use array2d::Array2D;
use itertools::{zip, Itertools};

#[derive(Debug)]
struct Game {
    draws: Vec<u8>,
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

        Ok(Game {
            draws: draw,
            boards,
        })
    }
}

#[derive(Debug)]
struct Board {
    array: Array2D<Mark<u8>>,
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
            array: Array2D::from_row_major(&lin, 5, 5),
        })
    }
}

impl Board {
    fn winner(&self) -> bool {
        let winning_column = self
            .array
            .columns_iter()
            .map(|column| column.into_iter().all(Mark::is_marked))
            .any(|winning_column| winning_column == true);

        let winning_row = self
            .array
            .rows_iter()
            .map(|row| row.into_iter().all(Mark::is_marked))
            .any(|winning_row| winning_row == true);

        // ^-_
        let descending = (0..5)
            .map(|co| self.array.get(co, co).unwrap())
            .all(Mark::is_marked);

        // _-^
        let ascending = zip(0..5, (0..5).rev())
            .map(|(row, column)| self.array.get(row, column).unwrap())
            .all(Mark::is_marked);

        winning_column || winning_row || descending || ascending
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

#[test]
fn winning() -> anyhow::Result<()> {
    let x = Mark::Marked(0);
    let o = Mark::Unmarked(0);
    let arr = [
        [x, o, o, x, x],
        [x, x, x, x, x],
        [x, x, x, o, x],
        [o, x, o, o, o],
        [x, o, o, x, x],
    ];
    let array = Array2D::from_iter_row_major(arr.into_iter().flatten(), 5, 5);
    println!("arr = {:?}", array);

    assert!(Board { array }.winner());
    Ok(())
}

fn find_winner(mut game: Game) {
    for draw in game.draws {
        for board in game.boards {
            board.array.mu
        }
    }
}
