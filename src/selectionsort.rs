use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [ sorted | not sorted ]
        for after_unsorted in 1..slice.len() {
            let unsorted = after_unsorted - 1;
            let smallest_in_rest = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(offset, _)| unsorted + offset)
                .expect("slice is nonempty");
            if unsorted != smallest_in_rest {
                slice.swap(unsorted, smallest_in_rest);
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    SelectionSort.sort(&mut things);
    assert_eq!(things, vec![1, 2, 3, 4, 5]);
}

#[test]
fn accepts_empty() {
    let mut empty: Vec<usize> = vec![];
    SelectionSort.sort(&mut empty);
}
