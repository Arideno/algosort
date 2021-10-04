use super::SortAlgo;

pub struct InsertionSort;

impl<T: Ord + Copy> SortAlgo<T> for InsertionSort {
    fn sort(&self, array: &mut [T]) {
        for i in 1..array.len() {
            let key = array[i];
            let mut j = (i - 1) as i64;

            while j >= 0 && array[j as usize] > key {
                array[(j + 1) as usize] = array[j as usize];
                j -= 1;
            }

            array[(j + 1) as usize] = key;
        }
    }
}