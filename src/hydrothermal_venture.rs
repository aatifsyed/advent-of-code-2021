//! # Next time
//! - Better integration with geo crate etc
//! - More efficient lattice points iterator
#[cfg(test)]
use crate::utils::CountOccurences;
use num::integer::gcd;
use recap::Recap;
use serde::Deserialize;

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

        if self.is_vertical() {
            return num::range_step_inclusive(self.y1, self.y2, dy.signum())
                .map(|y| Point { x: self.x1, y })
                .collect();
        }

        let divisor = gcd(dy, dx);
        let dy = dy / divisor;
        let dx = dx / divisor;

        num::range_step_inclusive(self.x1, self.x2, dx.signum())
            .enumerate()
            .map(|(count, x)| Point {
                x,
                y: self.y1 + (dy * count as isize),
            })
            .collect()
    }
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
