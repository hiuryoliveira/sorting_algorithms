// Module: tests

#[cfg(test)]
mod tests {

    use crate::sortable;

    #[test]
    fn test_merge_sort() {
        let mut arr = vec![4, 2, 9, 1, 7];
        let clone = arr.clone();
        sortable::merge_sort(&mut arr, 0, clone.len() - 1);
        assert_eq!(arr, vec![1, 2, 4, 7, 9]);
    }

    #[test]
    fn test_bubble_sort() {
        let mut arr = [4, 2, 9, 1, 7];
        sortable::bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 4, 7, 9]);
    }

    #[test]
    fn test_quick_sort() {
        let mut arr = [4, 2, 9, 1, 7];
        sortable::quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 4, 7, 9]);
    }
}


