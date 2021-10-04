use std::cmp::Ordering;
use std::mem;

mod selection_sort;
mod bubble_sort;
mod insertion_sort;
mod quick_sort;

pub use selection_sort::SelectionSort;
pub use bubble_sort::BubbleSort;
pub use insertion_sort::InsertionSort;
pub use quick_sort::QuickSort;

pub trait Sortable<T: Ord + Copy, A: SortAlgo<T>> {
    fn sort_with(&mut self, algo: A);
}

pub trait SortAlgo<T: Ord + Copy> {
    fn sort(&self, array: &mut [T]);
}

impl<T: Ord + Copy, A: SortAlgo<T>> Sortable<T, A> for [T] {
    fn sort_with(&mut self, algo: A) {
        algo.sort(self);
    }
}

fn swap<T>(x: &mut [T], i: usize, j: usize) {
    let (lo, hi) = match i.cmp(&j) {
        Ordering::Equal => return,
        Ordering::Less => (i, j),
        Ordering::Greater => (j, i),
    };

    let (init, tail) = x.split_at_mut(hi);
    mem::swap(&mut init[lo], &mut tail[0]);
}