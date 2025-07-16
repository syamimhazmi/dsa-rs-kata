fn linear_search(haystack: &[u32; 5], needle: u32) -> bool {
    for v in haystack.iter() {
        if *v == needle {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::linear_search;

    #[test]
    fn it_can_perform_linear_search() {
        let a: [u32; 5] = [1, 2, 3, 4, 5];

        let result = linear_search(&a, 5);

        assert!(result);
    }

    #[test]
    fn it_cant_perform_linear_search() {
        let a: [u32; 5] = [1, 2, 3, 4, 5];

        let result = linear_search(&a, 10);

        assert!(!result);
    }
}
