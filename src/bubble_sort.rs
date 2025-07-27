fn bubble_sort(arr: &mut [i32]) {
    let mut i = 0;

    while i < arr.len() {
        let mut j = 0;

        while j < arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
            j += 1;
        }

        i += 1;
    }
}

#[allow(dead_code)]
fn bubble_sort_idiomatic(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn it_can_sorted_an_array() {
        // Test Case 1: Normal unsorted array

        let mut arr = [64, 34, 25, 12, 22, 11, 90];
        let expected = [11, 12, 22, 25, 34, 64, 90];

        bubble_sort(&mut arr);

        assert_eq!(arr, expected);
    }

    #[test]
    fn it_can_run_edge_cases() {
        // Empty array
        let mut empty: [i32; 0] = [];
        bubble_sort(&mut empty);
        assert_eq!(empty, []);

        // Single element
        let mut single = [42];
        bubble_sort(&mut single);
        assert_eq!(single, [42]);

        // Two elements (unsorted)
        let mut two_elements = [5, 2];
        bubble_sort(&mut two_elements);
        assert_eq!(two_elements, [2, 5]);
    }

    #[test]
    fn it_sorted_special_array() {
        // Sorted array
        let mut already_sorted = [1, 2, 3, 4, 5];
        bubble_sort(&mut already_sorted);
        assert_eq!(already_sorted, [1, 2, 3, 4, 5]);

        // Reverse sorted array
        let mut reverse_sorted = [5, 4, 3, 2, 1];
        bubble_sort(&mut reverse_sorted);
        assert_eq!(reverse_sorted, [1, 2, 3, 4, 5]);

        // array with duplicates
        let mut with_duplicates = [3, 1, 4, 1, 5, 9, 2, 6, 5];
        bubble_sort(&mut with_duplicates);
        assert_eq!(with_duplicates, [1, 1, 2, 3, 4, 5, 5, 6, 9]);
    }
}
