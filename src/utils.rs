use std::{collections::HashMap, hash::Hash};

use array2d::Array2D;

pub trait CountOccurences<T> {
    fn count_occurences(self) -> HashMap<T, usize>;
}

impl<T, U> CountOccurences<T> for U
where
    U: IntoIterator<Item = T>,
    T: Hash + Eq,
{
    fn count_occurences(self) -> HashMap<T, usize> {
        self.into_iter()
            .fold(HashMap::new(), |mut counts, occurence| {
                *counts.entry(occurence).or_default() += 1;
                counts
            })
    }
}

pub struct Neighbours<'a, T: Clone> {
    array: &'a Array2D<T>,
    row: usize,
    column: usize,
}

pub struct NeighboursMut<'a, T: Clone> {
    array: &'a mut Array2D<T>,
    row: usize,
    column: usize,
}

struct Foo<T>(T);

impl Foo<&mut u8> {}

// TODO update smoke_basin to use this
pub trait Neighbouring<T: Clone> {
    fn neighbours(&self, row: usize, column: usize) -> Option<Neighbours<T>>;
    fn neighbours_mut(&mut self, row: usize, column: usize) -> Option<NeighboursMut<T>>;
}

impl<T: Clone> Neighbouring<T> for Array2D<T> {
    fn neighbours(&self, row: usize, column: usize) -> Option<Neighbours<T>> {
        self.get(row, column)?;
        Some(Neighbours {
            array: self,
            row,
            column,
        })
    }

    fn neighbours_mut(&mut self, row: usize, column: usize) -> Option<NeighboursMut<T>> {
        self.get(row, column)?;
        Some(NeighboursMut {
            array: self,
            row,
            column,
        })
    }
}
