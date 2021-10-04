use super::swap;
use super::SortAlgo;

pub struct SelectionSort;

impl<T: Ord + Copy> SortAlgo<T> for SelectionSort {
    fn sort(&self, array: &mut [T]) {
        for i in 0..array.len()-1 {
            let mut min_index = i;
            for j in i+1..array.len() {
                if array[j] < array[min_index] {
                    min_index = j;
                }
            }

            swap(array, min_index, i);
        }
    }
}