use crate::sorts::{SortAlgo, swap};

pub struct QuickSort;

impl<T: Ord + Copy> SortAlgo<T> for QuickSort {
    fn sort(&self, array: &mut [T]) {
        quick_sort(array, 0, array.len()-1);
    }
}

fn quick_sort<T: Ord + Copy>(array: &mut [T], low: usize, high: usize) {
    let mut i = low;
    let mut j = high;
    let pivot = array[(i + j) / 2];

    while i <= j {
        while array[i] < pivot {
            i += 1;
        }

        while array[j] > pivot {
            j -= 1;
        }

        if i <= j {
            swap(array, i, j);
            i += 1;
            j -= 1;
        }
    }

    if j > low {
        quick_sort(array, low, j);
    }

    if i < high {
        quick_sort(array, i, high);
    }
}