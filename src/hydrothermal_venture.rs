use std::{cmp, collections::HashMap, iter};

use recap::Recap;
use serde::Deserialize;

use crate::utils::CountOccurences;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}
/// Offload parsing to [`recap`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Recap)]
#[recap(regex = r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)")]
struct Segment {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Segment {
    fn is_vertical(&self) -> bool {
        self.x1 == self.x2
    }
    fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }
    fn xs(&self) -> Vec<usize> {
        if self.x2 > self.x1 {
            (self.x1..=self.x2).collect()
        } else {
            (self.x2..=self.x1).rev().collect()
        }
    }
    fn ys(&self) -> Vec<usize> {
        if self.y2 > self.y1 {
            (self.y1..=self.y2).collect()
        } else {
            (self.y2..=self.y1).rev().collect()
        }
    }
    fn points(&self) -> Vec<Point> {
        if self.is_horizontal() {
            self.xs()
                .into_iter()
                .map(|x| Point { x, y: self.y1 })
                .collect()
        } else if self.is_vertical() {
            self.ys()
                .into_iter()
                .map(|y| Point { x: self.x1, y })
                .collect()
        } else {
            assert_eq!(self.xs().len(), self.ys().len(), "must be diagonal");
            iter::zip(self.xs(), self.ys())
                .map(|(x, y)| Point { x, y })
                .collect()
        }
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
        .map(|s| s.points())
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
        .map(|s| s.points())
        .flatten()
        .count_occurences()
        .drain_filter(|_, count| *count >= 2)
        .count();
    assert_eq!(count, 6461);
}
