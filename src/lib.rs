mod sorts;

pub use sorts::SelectionSort;
pub use sorts::BubbleSort;
pub use sorts::InsertionSort;
pub use sorts::QuickSort;
pub use sorts::Sortable;
pub use sorts::SortAlgo;

#[cfg(test)]
mod tests {
    use super::sorts::{SelectionSort, BubbleSort, InsertionSort, Sortable, QuickSort};

    #[test]
    fn test_selection_sort() {
        let mut array = vec![3, 2, 1];
        array.sort_with(SelectionSort);
        assert_eq!(array, vec![1, 2, 3]);
    }

    #[test]
    fn test_bubble_sort() {
        let mut array = vec![3, 2, 1];
        array.sort_with(BubbleSort);
        assert_eq!(array, vec![1, 2, 3]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut array = vec![3, 2, 1];
        array.sort_with(InsertionSort);
        assert_eq!(array, vec![1, 2, 3]);
    }

    #[test]
    fn test_quick_sort() {
        let mut array = vec![3, 2, 1];
        array.sort_with(QuickSort);
        assert_eq!(array, vec![1, 2, 3]);
    }
}