use super::swap;
use super::SortAlgo;

pub struct BubbleSort;

impl<T: Ord + Copy> SortAlgo<T> for BubbleSort {
    fn sort(&self, array: &mut [T]) {
        for i in 0..array.len()-1 {
            for j in 0..array.len() - i - 1 {
                if array[j] > array[j+1] {
                    swap(array, j, j + 1);
                }
            }
        }
    }
}