use std::{cmp, collections::HashMap};

use geo::{coords_iter::CoordsIter, CoordNum, Coordinate, Line, LineString};
use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;

trait LineExt {
    fn is_horizontal(&self) -> bool;
    fn is_vertical(&self) -> bool;
    fn is_orthogonal(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }
    fn whole_coordinates(&self) -> Vec<Coordinate<usize>>;
}

impl LineExt for Line<usize> {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    /// Could be general
    fn whole_coordinates(&self) -> Vec<Coordinate<usize>> {
        if self.is_horizontal() {
            let min = cmp::min(self.start.x, self.end.x);
            let max = cmp::max(self.start.x, self.end.x);
            return (min..=max)
                .map(|x| Coordinate { x, y: self.start.y })
                .collect();
        }
        if self.is_vertical() {
            let min = cmp::min(self.start.y, self.end.y);
            let max = cmp::max(self.start.y, self.end.y);
            return (min..=max)
                .map(|y| Coordinate { x: self.start.x, y })
                .collect();
        }
        if self.slope() == 1 {}
        unimplemented!()
    }
}

/// Offload parsing to [`recap`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Recap)]
#[recap(regex = r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)")]
struct InputLine {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl From<InputLine> for Line<usize> {
    fn from(input_line: InputLine) -> Self {
        Self {
            start: Coordinate {
                x: input_line.x1,
                y: input_line.y1,
            },
            end: Coordinate {
                x: input_line.x2,
                y: input_line.y2,
            },
        }
    }
}

fn input() -> Vec<InputLine> {
    include_str!("inputs/2021/5.txt")
        .lines()
        .map(str::parse::<InputLine>)
        .collect::<Result<_, _>>()
        .expect("Couldn't parse input")
}

#[test]
fn part1() {
    let count = input()
        .into_iter()
        .map(Line::<usize>::from)
        .filter(LineExt::is_orthogonal)
        .map(|line| line.whole_coordinates())
        .flatten()
        .fold(HashMap::new(), |mut counter, coord| {
            *counter.entry(coord).or_insert(0usize) += 1;
            counter
        })
        .drain_filter(|_, count| *count >= 2)
        .count();
    assert_eq!(count, 6461);
}

#[test]
fn example() -> anyhow::Result<()> {
    let count = "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2"
        .lines()
        .map(str::parse::<InputLine>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(Line::<usize>::from)
        .filter(LineExt::is_orthogonal)
        .map(|line| line.whole_coordinates())
        .flatten()
        .fold(HashMap::new(), |mut counter, coord| {
            *counter.entry(coord).or_insert(0usize) += 1;
            counter
        })
        .drain_filter(|_, count| *count >= 2)
        .count();
    assert_eq!(count, 5);
    Ok(())
}

#[test]
fn test_whole_coords() -> anyhow::Result<()> {
    let line = "9,7 -> 7,7".parse::<InputLine>().map(Line::<usize>::from)?;
    println!("line = {:?}", line);
    dbg!(line.whole_coordinates());
    Ok(())
}

#[test]
fn test_slope() -> anyhow::Result<()> {
    let line = "2,6 -> 0,0".parse::<InputLine>().map(Line::<usize>::from)?;
    println!("line = {:?}", line);
    dbg!(line.dy());
    dbg!(line.dx());

    Ok(())
}
