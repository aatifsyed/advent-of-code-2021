use std::{collections::HashMap, hash::Hash};

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
