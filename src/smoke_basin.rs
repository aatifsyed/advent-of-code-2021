//! This is a mess, I got lazy
use std::collections::HashSet;

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
    up: Option<T>,
    left: Option<T>,
    right: Option<T>,
    down: Option<T>,
}

impl<T: Num + Ord + Copy> Kernel<T> {
    fn neighbours(&self) -> [Option<T>; 4] {
        [self.up, self.left, self.right, self.down]
    }
    fn is_low(&self) -> bool {
        self.neighbours()
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
    fn flood_fill(&self, row: usize, column: usize) -> HashSet<(usize, usize)>;
}

impl ArrayExt<u32> for Array2D<u32> {
    fn kernel_for(&self, row: usize, column: usize) -> Option<Kernel<u32>> {
        let item = self.get(row, column)?;
        let mut up = None;
        let mut left = None;

        if row > 0 {
            up = self.get(row - 1, column).map(Clone::clone);
        }
        if column > 0 {
            left = self.get(row, column - 1).map(Clone::clone);
        }

        let right = self.get(row, column + 1).map(Clone::clone);
        let down = self.get(row + 1, column).map(Clone::clone);

        Some(Kernel {
            item: item.clone(),
            up,
            left,
            right,
            down,
        })
    }

    fn flood_fill(&self, row: usize, column: usize) -> HashSet<(usize, usize)> {
        let mut already_visted = HashSet::new();
        flood_fill_inner(self, &mut already_visted, row, column);
        already_visted
    }
}

fn flood_fill_inner(
    array: &Array2D<u32>,
    already_visted: &mut HashSet<(usize, usize)>,
    row: usize,
    column: usize,
) {
    match already_visted.insert((row, column)) {
        true => (),
        false => return,
    }

    if let Some(kernel) = array.kernel_for(row, column) {
        if let Some(up) = kernel.up {
            if up < 9 {
                flood_fill_inner(array, already_visted, row - 1, column)
            }
        }
        if let Some(down) = kernel.down {
            if down < 9 {
                flood_fill_inner(array, already_visted, row + 1, column)
            }
        }
        if let Some(left) = kernel.left {
            if left < 9 {
                flood_fill_inner(array, already_visted, row, column - 1)
            }
        }
        if let Some(right) = kernel.right {
            if right < 9 {
                flood_fill_inner(array, already_visted, row, column + 1)
            }
        }
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
    let height_map = parse(input)?;
    let mut basin_sizes = Vec::new();
    for row in 0..height_map.num_rows() {
        for column in 0..height_map.num_columns() {
            let kernel = height_map.kernel_for(row, column).expect("Valid index");
            if kernel.is_low() {
                let size = height_map.flood_fill(row, column).len();
                basin_sizes.push(size)
            }
        }
    }
    basin_sizes.sort();
    let top3 = basin_sizes
        .into_iter()
        .rev()
        .take(3)
        .reduce(|a, b| a * b)
        .context("Not enough basins")?;
    Ok(top3)
}

benchtest::benchtest! {
    part1: do_part1(test::black_box(INPUT)).unwrap() => 478,
    part2: do_part2(test::black_box(INPUT)).unwrap() => 1327014
}
