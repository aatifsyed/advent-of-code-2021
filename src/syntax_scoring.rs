use std::str::FromStr;

use anyhow::{bail, Context};
use itertools::Itertools;

use crate::treachery_of_whales::median;

extern crate test;

const INPUT: &str = include_str!("./inputs/2021/10.txt");

#[derive(Debug)]
enum Line {
    Illegal(char),
    Incomplete(usize),
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();
        for char in s.chars() {
            match char {
                '(' | '[' | '{' | '<' => stack.push(char),
                ')' => match stack.pop() {
                    Some(popped) if popped == '(' => (),
                    Some(_) | None => return Ok(Line::Illegal(char)),
                },
                ']' => match stack.pop() {
                    Some(popped) if popped == '[' => (),
                    Some(_) | None => return Ok(Line::Illegal(char)),
                },
                '}' => match stack.pop() {
                    Some(popped) if popped == '{' => (),
                    Some(_) | None => return Ok(Line::Illegal(char)),
                },
                '>' => match stack.pop() {
                    Some(popped) if popped == '<' => (),
                    Some(_) | None => return Ok(Line::Illegal(char)),
                },
                _ => bail!("Unexpected character: {}", char),
            }
        }
        let score = stack
            .into_iter()
            .rev()
            .fold(0, |score, opening| match opening {
                '(' => score * 5 + 1,
                '[' => score * 5 + 2,
                '{' => score * 5 + 3,
                '<' => score * 5 + 4,
                _ => unreachable!(),
            });
        Ok(Line::Incomplete(score))
    }
}

fn parse(input: &str) -> anyhow::Result<Vec<Line>> {
    input.lines().map(str::parse::<Line>).collect()
}

fn do_part1(input: &str) -> anyhow::Result<usize> {
    let lines = parse(input)?;

    let score = lines.into_iter().fold(0, |acc, el| match el {
        Line::Illegal(closing) => match closing {
            ')' => acc + 3,
            ']' => acc + 57,
            '}' => acc + 1197,
            '>' => acc + 25137,
            _ => unreachable!(),
        },
        Line::Incomplete(_) => acc,
    });
    Ok(score)
}
fn do_part2(input: &str) -> anyhow::Result<usize> {
    let lines = parse(input)?;
    let mut scores = lines
        .into_iter()
        .filter_map(|line| match line {
            Line::Illegal(_) => None,
            Line::Incomplete(score) => Some(score),
        })
        .collect_vec();
    Ok(*median(&mut scores).context("No median")?)
}

benchtest::benchtest! {
    part1: do_part1(test::black_box(INPUT)).unwrap() => 243939,
    part2: do_part2(test::black_box(INPUT)).unwrap() => 2421222841
}
