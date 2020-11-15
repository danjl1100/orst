//! Comparison of basic sorting implementations
//!
//! Following along with [jon gjengset](https://thesquareplanet.com/)'s video [Crust of Rust: Sorting Algorithms](https://www.youtube.com/watch?v=h4RkCyJyXmM)
pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord + std::fmt::Debug + Clone;
}

mod bubblesort;
mod insertionsort;
mod quicksort;
mod selectionsort;

pub use bubblesort::BubbleSort;
pub use insertionsort::InsertionSort;
pub use quicksort::QuickSort;
pub use selectionsort::SelectionSort;

pub struct StdSorter;
impl Sorter for StdSorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_works() {
        let mut things = vec![4, 2, 3, 1];
        StdSorter.sort(&mut things);
        assert_eq!(things, vec![1, 2, 3, 4]);
    }
}
