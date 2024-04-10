fn main() {
    let mut data = vec![5, 3, 2, 4, 1];
    println!("Before: {:?}", data);
    // sortable::insertion_sort(&mut data);
    // sortable::selection_sort(&mut data);
    // sortable::merge_sort(&mut data, 0, data.len() - 1);
    // sortable::bubble_sort(&mut data);
    sortable::quick_sort(&mut data);
    println!("After: {:?}", data);
}


mod sortable {
    // Insertion Sort
    pub(crate) fn insertion_sort<T: Ord>(data: &mut [T]) {
        for i in 1..data.len() {
            let mut j = i;
            while j > 0 && data[j - 1] > data[j] {
                data.swap(j - 1, j);
                j -= 1;
            }
        }
    }

    // Selection Sort
    pub(crate) fn selection_sort(data : &mut Vec<i32>) {
        let n = data.len();
        for i in 0..n {
            let mut min = i;
            for j in i+1..n {
                if data[j] < data[min] {
                    min = j;
                }
            }
            data.swap(i, min);
        }
    }


    // Merge Sort
    fn merge(data: &mut Vec<i32>, left: usize, middle: usize, right: usize) {
        let size_left = middle - left + 1;
        let size_rigth = right - middle;

        let mut left_data = vec![0; size_left];
        let mut right_data = vec![0; size_rigth];

        for i in 0..size_left {
            left_data[i] = data[left + i];
        }

        for i in 0..size_rigth {
            right_data[i] = data[middle + 1 + i];
        }

        let mut i = 0;
        let mut j = 0;
        let mut k = left;

        while i < size_left && j < size_rigth {
            if left_data[i] <= right_data[j] {
                data[k] = left_data[i];
                i += 1;
            } else {
                data[k] = right_data[j];
                j += 1;
            }
            k += 1;
        }

        while i < size_left {
            data[k] = left_data[i];
            i += 1;
            k += 1;
        }

        while j < size_rigth {
            data[k] = right_data[j];
            j += 1;
            k += 1;
        }
    }

    pub(crate) fn merge_sort(arr: &mut Vec<i32>, start: usize, end: usize) {
        if start < end {
            let mid = (start + end) / 2;
            merge_sort(arr, start, mid);
            merge_sort(arr, mid + 1, end);
            merge(arr, start, mid, end);
        }
    }

    // Bubble Sort
    pub(crate) fn bubble_sort<T: Ord>(sortable: &mut [T]) {
        let n = sortable.len();
        for i in 0..n {
            for j in 0..n - i - 1 {
                if sortable[j] > sortable[j + 1] {
                    sortable.swap(j, j + 1);
                }
            }
        }
    }

    // Quick Sort
    pub(crate) fn quick_sort(data: &mut [i32]) {
        if data.len() <= 1 {
            return;
        }
        let pivot = data.len() / 2;
        let mut left = 0;
        let right = data.len() - 1;
        data.swap(pivot, right);
        for i in 0..data.len() {
            if data[i] < data[right] {
                data.swap(i, left);
                left += 1;
            }
        }
        data.swap(left, right);
        quick_sort(&mut data[0..left]);
        quick_sort(&mut data[left + 1..]);
    }
}


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
