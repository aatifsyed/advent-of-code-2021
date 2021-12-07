use anyhow::{bail, Context};
use std::str::FromStr;

extern crate test;

const INPUT: &str = include_str!("./inputs/2021/2.txt");

fn parse(input: &str) -> anyhow::Result<Vec<DiveInstruction>> {
    input
        .lines()
        .map(str::parse::<DiveInstruction>)
        .collect::<Result<_, _>>()
        .context("Invalid input format")
}

fn do_part1(input: &str) -> anyhow::Result<usize> {
    let pos = follow_course(parse(input)?);
    Ok(pos.horizontal * pos.depth)
}
fn do_part2(input: &str) -> anyhow::Result<usize> {
    let pos = follow_course_aim(parse(input)?);
    Ok(pos.horizontal * pos.depth)
}

benchtest::benchtest! {
    part1: do_part1(test::black_box(INPUT)).unwrap() => 1524750,
    part2: do_part2(test::black_box(INPUT)).unwrap() => 1592426537
}

enum DiveInstruction {
    Forward(usize),
    Up(usize),
    Down(usize),
}

impl FromStr for DiveInstruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let word = split.next().context("No word")?;
        let num = split.next().context("No num")?.parse()?;
        let dive_instruction = match word {
            "forward" => Self::Forward(num),
            "up" => Self::Up(num),
            "down" => Self::Down(num),
            other => bail!("Invalid instruction: {}", other),
        };
        Ok(dive_instruction)
    }
}

#[derive(Debug, Default)]
struct Position {
    horizontal: usize,
    depth: usize,
}

fn follow_course(input: impl IntoIterator<Item = DiveInstruction>) -> Position {
    input
        .into_iter()
        .fold(Position::default(), |mut pos, instruction| {
            match instruction {
                DiveInstruction::Forward(by) => pos.horizontal += by,
                DiveInstruction::Up(by) => pos.depth -= by,
                DiveInstruction::Down(by) => pos.depth += by,
            }
            pos
        })
}

fn follow_course_aim(input: impl IntoIterator<Item = DiveInstruction>) -> Position {
    let (_, pos) = input.into_iter().fold(
        (0, Position::default()),
        |(mut aim, mut pos), instruction| {
            match instruction {
                DiveInstruction::Forward(by) => {
                    pos.horizontal += by;
                    pos.depth += aim * by;
                }
                DiveInstruction::Up(by) => aim -= by,
                DiveInstruction::Down(by) => aim += by,
            };
            (aim, pos)
        },
    );
    pos
}
