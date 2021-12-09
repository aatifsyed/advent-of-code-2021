use anyhow::{ensure, Context};
use array2d::Array2D;
use num::Num;

fn parse(input: &str) -> anyhow::Result<Array2D<u32>> {
    let v = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10))
                .collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<_>>>()
        .context("Invalid input")?;
    ensure!(
        v.windows(2).all(|vs| vs[0].len() == vs[1].len()),
        "Rows have inconsistent length"
    );
    Ok(Array2D::from_rows(&v))
}

struct Kernel<T> {
    item: T,
    neighbours: [Option<T>; 8],
}

impl<T: Num + Ord + Copy> Kernel<T> {
    fn is_low(&self) -> bool {
        self.neighbours
            .iter()
            .filter_map(|n| n.as_ref())
            .all(|n| *n > self.item)
    }
    fn risk_level(&self) -> T {
        self.item + T::one()
    }
}

trait ArrayExt<T> {
    fn kernel_for(&self, row: usize, column: usize) -> Option<Kernel<T>>;
}

impl<T: Copy> ArrayExt<T> for Array2D<T> {
    fn kernel_for(&self, row: usize, column: usize) -> Option<Kernel<T>> {
        let item = *self.get(row, column)?;
        let mut neighbours = [None; 8];

        if row > 0 && column > 0 {
            // Could also do wrapping_sub because we're unlikely to have a 2DArray that big
            neighbours[0] = self.get(row - 1, column - 1).map(Clone::clone); // Top left
            neighbours[5] = self.get(row - 1, column - 1).map(Clone::clone); // Bottom left
        }
        if row > 0 {
            neighbours[1] = self.get(row - 1, column).map(Clone::clone); // Top middle
            neighbours[2] = self.get(row - 1, column + 1).map(Clone::clone); // Top right
        }
        if column > 0 {
            neighbours[3] = self.get(row, column - 1).map(Clone::clone); // Left
        }

        neighbours[4] = self.get(row, column + 1).map(Clone::clone); // Right
        neighbours[6] = self.get(row + 1, column).map(Clone::clone); // Bottom middle
        neighbours[7] = self.get(row + 1, column + 1).map(Clone::clone); // Bottom right

        Some(Kernel { item, neighbours })
    }
}

extern crate test;

const INPUT: &str = include_str!("./inputs/2021/9.txt");

fn do_part1(input: &str) -> anyhow::Result<u32> {
    let arr = parse(input)?;
    let mut total_risk_level = 0;
    for row in 0..arr.num_rows() {
        for column in 0..arr.num_columns() {
            let kernel = arr.kernel_for(row, column).expect("Valid index");
            if kernel.is_low() {
                total_risk_level += kernel.risk_level()
            }
        }
    }
    Ok(total_risk_level)
}
fn do_part2(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

benchtest::benchtest! {
    part1: do_part1(test::black_box(INPUT)).unwrap() => 478,
    part2: do_part2(test::black_box(INPUT)).unwrap() => 0
}
