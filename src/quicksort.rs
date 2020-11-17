use super::Sorter;

pub struct QuickSort;

fn quicksort<T: Ord + std::fmt::Debug + Clone>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        // not needed
        // 2 => {
        //     if slice[0] > slice[1] {
        //         slice.swap(0, 1);
        //     }
        //     return
        // }
        _ => {}
    };

    let (pivot, rest) = slice.split_first_mut().expect("slice is not empty");
    let mut left = 0;
    let mut right = rest.len();
    while left < right {
        let left_correct = &rest[left] < pivot;
        let right_correct = &rest[right - 1] >= pivot; // equality here for sorting stability
        if left_correct {
            // already on the correct side
            left += 1;
        }
        if right_correct {
            // right already on the correct side
            right -= 1;
        }

        if !left_correct && !right_correct {
            // left holds a right, and right holds a left, swap them.
            rest.swap(left, right - 1);
            left += 1;
            right -= 1;
        }
    }

    // re-align left to account for the pivot at 0
    let pivot_location = {
        let left_global = left + 1;
        left_global - 1
    };

    // place the pivot at its final location (if not already there)
    if pivot_location > 0 {
        slice.swap(0, pivot_location);
    } else {
    }

    // split_at_mut(mid: usize) -> (&mut [..mid), &mut [mid..])
    let (left, right) = slice.split_at_mut(pivot_location);
    if !left.is_empty() && !right.is_empty() {
        assert!(
            left.last() <= right.first(),
            format!("left.last= {:?} {:?}", left.last(), right.first())
        );
    }
    if !left.is_empty() {
        quicksort(left);
    } else {
    }
    if !right.is_empty() {
        quicksort(&mut right[1..]);
    } else {
    }
}

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord + std::fmt::Debug + Clone,
    {
        // [ unsorted | pivot | unsorted ]
        quicksort(slice);
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    QuickSort.sort(&mut things);
    assert_eq!(things, vec![1, 2, 3, 4, 5]);
}

#[test]
fn longer() {
    let mut things = vec![6, 5, 4, 3, 2, 1];
    QuickSort.sort(&mut things);
    assert_eq!(things, vec![1, 2, 3, 4, 5, 6]);
}
