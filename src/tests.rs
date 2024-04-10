// Module: tests

#[cfg(test)]
mod tests {
    use crate::sortable;

    #[test]
    fn test_insertion_sort() {
        let mut data = [5, 2, 4, 6, 1, 3];
        sortable::insertion_sort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_selection_sort() {
        let mut data = vec![4, 2, 3, 19, 29, 14, 7, 84, 9, 3, 77, 45, 37, 2, 84, 88, 30, 1];
        sortable::selection_sort(&mut data);
        assert_eq!(data, vec![1, 2, 2, 3, 3, 4, 7, 9, 14, 19, 29, 30, 37, 45, 77, 84, 84, 88]);
    }

    #[test]
    fn test_merge_sort() {
        let mut sortable = vec![12, 11, 13, 5, 19, 189, 38, 18, 37, 60, 90, 6, 7];
        let length = sortable.len();
        sortable::merge_sort(&mut sortable, 0, length - 1);
        assert_eq!(sortable, vec![5, 6, 7, 11, 12, 13, 18, 19, 37, 38, 60, 90, 189]);
    }

    #[test]
    fn test_bubble_sort() {
        let mut sortable = [64, 34, 25, 12, -18, 22, 11, 18, 26, 69, 9, 90];
        sortable::bubble_sort(&mut sortable);
        assert_eq!(sortable, [-18, 9, 11, 12, 18, 22, 25, 26, 34, 64, 69, 90]);
    }

    #[test]
    fn test_quick_sort() {
        let mut data = [4, 2, 3, 1];
        sortable::quick_sort(&mut data);
        assert_eq!(data, [1, 2, 3, 4]);

        let mut data = [5, 1, 9, 3, 7];
        sortable::quick_sort(&mut data);
        assert_eq!(data, [1, 3, 5, 7, 9]);

        let mut data = [10, 8, 6, 4, 2];
        sortable::quick_sort(&mut data);
        assert_eq!(data, [2, 4, 6, 8, 10]);
    }
}


