// this function to check when point low and high is meet
fn qs(arr: &mut [i32], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let pivot_idx = partition(arr, lo, hi);

    // we recurse to two sides of array but not included with pivot index
    // Only recurse left if there are elements to the left
    if pivot_idx > lo {
        qs(arr, lo, pivot_idx - 1);
    }

    // Only recurse right if there are elements to the right
    if pivot_idx < hi {
        qs(arr, pivot_idx + 1, hi);
    }
}

// this will return a number which will be repesentation of pivot index
// where did we end up splitting the array
fn partition(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let pivot = arr[hi];
    let mut idx = lo;

    for i in lo..hi {
        println!("{i:#}");
        if arr[i] <= pivot {
            if idx != i {
                // manual way of swapping array
                // let tmp = arr[i];
                // arr[i] = arr[idx];
                // arr[idx] = tmp;
                arr.swap(i, idx);
            }
            idx += 1;
        }
    }

    // manual way of swapping array
    // arr[hi] = arr[idx];
    // arr[idx] = pivot;
    arr.swap(hi, idx);

    idx
}

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        // Handle empty and single-element arrays
        return;
    }
    qs(arr, 0, arr.len() - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_unsorted_array() {
        let mut arr = [64, 34, 25, 12, 22, 11, 90];
        quick_sort(&mut arr);
        assert_eq!(arr, [11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_already_sorted_array() {
        let mut arr = [1, 2, 3, 4, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted_array() {
        let mut arr = [9, 7, 5, 3, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_array_with_duplicates() {
        let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 1, 2, 3, 4, 5, 5, 6, 9]);
    }

    #[test]
    fn test_single_element() {
        let mut arr = [42];
        quick_sort(&mut arr);
        assert_eq!(arr, [42]);
    }

    #[test]
    fn test_empty_array() {
        let mut arr: [i32; 0] = [];
        quick_sort(&mut arr);
        assert_eq!(arr, []);
    }
}
