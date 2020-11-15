use orst::*;

use rand::prelude::*;
use std::cell::Cell;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone)]
struct SortEvaluator<T> {
    t: T,
    cmps: Rc<Cell<usize>>,
}
impl<T> SortEvaluator<T> {
    fn count(&self) {
        self.cmps.set(self.cmps.get() + 1);
    }
}
impl<T: Debug> Debug for SortEvaluator<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.t.fmt(f)
    }
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.count();
        self.t == other.t
    }
}
impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.count();
        self.t.partial_cmp(&other.t)
    }
}
impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count();
        self.t.cmp(&other.t)
    }
}

fn main() {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));
    for &n in &[0, 1, 10, 100, 1_000, 10_000] {
        for _ in 0..10 {
            let mut values = Vec::with_capacity(n);
            for _ in 0..n {
                values.push(SortEvaluator {
                    t: rand.gen::<usize>(),
                    cmps: Rc::clone(&counter),
                });
            }

            print(n, "bubble", bench(BubbleSort, &values, &counter));
            print(
                n,
                "insertion-smart",
                bench(InsertionSort { smart: true }, &values, &counter),
            );
            print(
                n,
                "insertion-dumb",
                bench(InsertionSort { smart: false }, &values, &counter),
            );
            print(n, "selection", bench(SelectionSort, &values, &counter));
            print(n, "quick", bench(QuickSort, &values, &counter));
            print(n, "stdlib", bench(StdSorter, &values, &counter));
        }
    }
}

fn print(n: usize, label: &str, stats: (usize, f64)) {
    let (count, took) = stats;
    println!("{} {} {} {}", label, n, count, took);
}

fn bench<T: Ord + Clone + std::fmt::Debug, S: Sorter>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) -> (usize, f64) {
    let mut values: Vec<_> = values.to_vec();
    counter.set(0);
    let time = std::time::Instant::now();
    sorter.sort(&mut values);
    let took = time.elapsed();
    let count = counter.get();
    for i in 1..values.len() {
        assert!(
            values[i] >= values[i - 1],
            format!("{:?} >= {:?} failed", values[i], values[i - 1])
        );
    }
    (count, took.as_secs_f64())
}
