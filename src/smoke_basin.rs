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
    top_left: Option<T>,
    top_middle: Option<T>,
    top_right: Option<T>,
    left: Option<T>,
    right: Option<T>,
    bottom_left: Option<T>,
    bottom_middle: Option<T>,
    bottom_right: Option<T>,
}

impl<T: Num + Ord + Copy> Kernel<T> {
    fn direct_neighbours(&self) -> [Option<T>; 4] {
        [self.top_middle, self.left, self.right, self.bottom_middle]
    }
    fn all_neighbours(&self) -> [Option<T>; 8] {
        [
            self.top_left,
            self.top_middle,
            self.top_right,
            self.left,
            self.right,
            self.bottom_left,
            self.bottom_middle,
            self.bottom_right,
        ]
    }
    fn is_low(&self) -> bool {
        self.all_neighbours()
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
        let mut top_left = None;
        let mut top_middle = None;
        let mut top_right = None;
        let mut left = None;
        let mut bottom_left = None;

        if row > 0 && column > 0 {
            // Could also do wrapping_sub because we're unlikely to have a 2DArray that big
            top_left = self.get(row - 1, column - 1).map(Clone::clone);
            bottom_left = self.get(row - 1, column - 1).map(Clone::clone);
        }
        if row > 0 {
            top_middle = self.get(row - 1, column).map(Clone::clone);
            top_right = self.get(row - 1, column + 1).map(Clone::clone);
        }
        if column > 0 {
            left = self.get(row, column - 1).map(Clone::clone);
        }

        let right = self.get(row, column + 1).map(Clone::clone);
        let bottom_middle = self.get(row + 1, column).map(Clone::clone);
        let bottom_right = self.get(row + 1, column + 1).map(Clone::clone);

        Some(Kernel {
            item,
            top_left,
            top_middle,
            top_right,
            left,
            right,
            bottom_left,
            bottom_middle,
            bottom_right,
        })
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
                let mut fill = 1;
                count_fill(&mut fill, row, column, &height_map);
                basin_sizes.push(fill);
            }
        }
    }

    Ok(0)
}

fn count_fill(count: &mut usize, row: usize, column: usize, array: &Array2D<u32>) {
    let kernel = array.kernel_for(row, column).unwrap();
    match kernel.top_middle {
        Some(9) | None => (),
        Some(_) => {
            *count += 1;
            count_fill(count, row, column, array)
        }
    }
}

benchtest::benchtest! {
    part1: do_part1(test::black_box(INPUT)).unwrap() => 478,
    part2: do_part2(test::black_box(INPUT)).unwrap() => 0
}
