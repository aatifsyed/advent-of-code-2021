/// # Next time
/// - Better integration with geo crate etc
/// - More efficient lattice points iterator
use std::{cmp, collections::HashMap, iter};

use num::{integer::gcd, rational::Ratio};
use recap::Recap;
use serde::Deserialize;

use crate::utils::CountOccurences;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}
/// Offload parsing to [`recap`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Recap)]
#[recap(regex = r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)")]
struct Segment {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
}

impl Segment {
    fn is_vertical(&self) -> bool {
        self.x1 == self.x2
    }
    fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }
    fn lattice_points(&self) -> Vec<Point> {
        let dy = self.y2 - self.y1;
        let dx = self.x2 - self.x1;

        let start = Point {
            x: self.x1,
            y: self.y1,
        };
        let end = Point {
            x: self.x2,
            y: self.y2,
        };

        if self.is_vertical() {
            return iter::successors(Some(start), |previous| match *previous == end {
                true => None,
                false => Some(Point {
                    x: previous.x,
                    y: previous.y + dy.signum(),
                }),
            })
            .collect();
        }

        let divisor = gcd(dy, dx);
        let dy = dy / divisor;
        let dx = dx / divisor;

        iter::successors(Some(start), |previous| match *previous == end {
            true => None,
            false => Some(Point {
                x: previous.x + dx,
                y: previous.y + dy,
            }),
        })
        .collect()
    }
}

#[test]
fn test_lattice_points() -> anyhow::Result<()> {
    let points = "9,7 -> 7,7".parse::<Segment>()?.lattice_points();
    println!("points = {:?}", points);
    Ok(())
}

fn input() -> Vec<Segment> {
    include_str!("inputs/2021/5.txt")
        .lines()
        .map(str::parse::<Segment>)
        .collect::<Result<_, _>>()
        .expect("Couldn't parse input")
}

#[test]
fn part1() {
    let count = input()
        .into_iter()
        .filter(|segment| segment.is_horizontal() || segment.is_vertical())
        .map(|s| s.lattice_points())
        .flatten()
        .count_occurences()
        .drain_filter(|_, count| *count >= 2)
        .count();
    assert_eq!(count, 6461);
}

#[test]
fn part2() {
    let count = input()
        .into_iter()
        .map(|s| s.lattice_points())
        .flatten()
        .count_occurences()
        .drain_filter(|_, count| *count >= 2)
        .count();
    assert_eq!(count, 18065);
}
