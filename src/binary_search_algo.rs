fn binary_search(hasystack: &[u32; 5], needle: u32) -> bool {
    let mut low = 0;
    let mut high = hasystack.len();

    while low < high {
        let middle = low + (high - low) / 2;
        let value = hasystack[middle];

        #[allow(clippy::comparison_chain)]
        if value == needle {
            return true;
        } else if value > needle {
            high = middle;
        } else {
            low = middle + 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn it_can_perform_binary_search() {
        let arr: [u32; 5] = [1, 2, 3, 4, 5];

        let result = binary_search(&arr, 5);

        assert!(result);
    }

    #[test]
    fn it_cant_find_number_6_in_binary_search() {
        let arr: [u32; 5] = [1, 2, 3, 4, 5];

        let result = binary_search(&arr, 6);

        assert!(!result);
    }
}
