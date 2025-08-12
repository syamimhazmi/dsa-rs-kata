pub struct MinHeap {
    pub data: Vec<f64>,
}

impl MinHeap {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn insert(&mut self, value: f64) {
        self.data.push(value);

        if self.data.len() > 1 {
            self.heapify_up(self.data.len() - 1);
        }
    }

    pub fn peek(&self) -> Option<f64> {
        self.data.first().copied()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn delete(&mut self) -> Option<f64> {
        if self.data.is_empty() {
            return None;
        }

        let min_value = self.data[0];
        let last = self.data.pop().unwrap();

        // if there are still elements, move the last element to root and heapify down
        if !self.data.is_empty() {
            self.data[0] = last;
            self.heapify_down(0);
        }

        Some(min_value)
    }

    fn heapify_down(&mut self, mut idx: usize) {
        let len = self.data.len();

        loop {
            let left_child = self.left_child(idx);
            let right_child = self.right_child(idx);
            let mut smallest = idx;

            // find the smallest among parent and its children
            if left_child < len && self.data[left_child] < self.data[smallest] {
                smallest = left_child;
            }

            if right_child < len && self.data[right_child] < self.data[smallest] {
                smallest = right_child;
            }

            // if current element is already the smallest, we're done
            if smallest == idx {
                break;
            }

            // swap with the smallest child and continue
            self.data.swap(idx, smallest);
            idx = smallest;
        }
    }

    fn heapify_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent_idx = self.parent(idx);

            if self.data[parent_idx] <= self.data[idx] {
                break;
            }

            self.data.swap(idx, parent_idx);
            idx = parent_idx;
        }
    }

    fn parent(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn left_child(&self, idx: usize) -> usize {
        2 * idx + 1
    }

    fn right_child(&self, idx: usize) -> usize {
        2 * idx + 2
    }

    #[cfg(test)]
    fn is_valid_heap(&self) -> bool {
        for i in 0..self.data.len() {
            let left = self.left_child(i);
            let right = self.right_child(i);

            if left < self.data.len() && self.data[i] > self.data[left] {
                return false;
            }

            if right < self.data.len() && self.data[i] > self.data[right] {
                return false;
            }
        }

        true
    }
}

impl Default for MinHeap {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<f64>> for MinHeap {
    fn from(mut values: Vec<f64>) -> Self {
        // build heap from bottom up (more efficient than inserting one by one)
        if values.is_empty() {
            return MinHeap::new();
        }

        let mut heap = MinHeap { data: values };

        // start from the last parent node and heapify down
        let last_parent = (heap.data.len() - 1) / 2;

        for i in (0..=last_parent).rev() {
            heap.heapify_down(i);
        }

        heap
    }
}

#[cfg(test)]
mod tests {
    use crate::min_heap::MinHeap;

    #[test]
    fn test_new_heap() {
        let heap = MinHeap::new();
        assert!(heap.is_empty());
        assert_eq!(heap.len(), 0);
        assert_eq!(heap.peek(), None);
    }

    #[test]
    fn test_single_element() {
        let mut heap = MinHeap::new();
        heap.insert(42.0);

        assert!(!heap.is_empty());
        assert_eq!(heap.len(), 1);
        assert_eq!(heap.peek(), Some(42.0));
        assert_eq!(heap.delete(), Some(42.0));
        assert!(heap.is_empty());
    }

    #[test]
    fn test_multiple_insertions() {
        let mut heap = MinHeap::new();
        let values = vec![5.0, 3.0, 8.0, 1.0, 9.0, 2.0];

        for value in values {
            heap.insert(value);
            assert!(heap.is_valid_heap());
        }

        assert_eq!(heap.len(), 6);
        assert_eq!(heap.peek(), Some(1.0));
    }

    #[test]
    fn test_pop_in_sorted_order() {
        let mut heap = MinHeap::new();
        let values = vec![5.0, 3.0, 8.0, 1.0, 9.0, 2.0, 7.0, 4.0, 6.0];

        for value in values {
            heap.insert(value);
        }

        let mut result = Vec::new();
        while let Some(min) = heap.delete() {
            result.push(min);
            assert!(heap.is_valid_heap());
        }

        assert_eq!(result, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        assert!(heap.is_empty());
    }

    #[test]
    fn test_with_duplicates() {
        let mut heap = MinHeap::new();
        let values = vec![3.0, 1.0, 3.0, 1.0, 2.0, 2.0];

        for value in values {
            heap.insert(value);
        }

        let mut result = Vec::new();
        while let Some(min) = heap.delete() {
            result.push(min);
        }

        assert_eq!(result, vec![1.0, 1.0, 2.0, 2.0, 3.0, 3.0]);
    }

    #[test]
    fn test_with_negative_numbers() {
        let mut heap = MinHeap::new();
        let values = vec![-5.0, 3.0, -1.0, 8.0, -10.0, 0.0];

        for value in values {
            heap.insert(value);
        }

        assert_eq!(heap.peek(), Some(-10.0));
        assert_eq!(heap.delete(), Some(-10.0));
        assert_eq!(heap.delete(), Some(-5.0));
        assert_eq!(heap.delete(), Some(-1.0));
        assert_eq!(heap.delete(), Some(0.0));
    }

    #[test]
    fn test_from_vector() {
        let values = vec![5.0, 3.0, 8.0, 1.0, 9.0, 2.0];
        let heap = MinHeap::from(values);

        assert!(heap.is_valid_heap());
        assert_eq!(heap.len(), 6);
        assert_eq!(heap.peek(), Some(1.0));
    }

    #[test]
    fn test_clear() {
        let mut heap = MinHeap::new();
        heap.insert(1.0);
        heap.insert(2.0);
        heap.insert(3.0);

        assert_eq!(heap.len(), 3);
        heap.clear();
        assert!(heap.is_empty());
        assert_eq!(heap.peek(), None);
    }

    #[test]
    fn test_peek_doesnt_modify_heap() {
        let mut heap = MinHeap::new();
        heap.insert(3.0);
        heap.insert(1.0);
        heap.insert(2.0);

        let original_len = heap.len();
        assert_eq!(heap.peek(), Some(1.0));
        assert_eq!(heap.len(), original_len);
        assert!(heap.is_valid_heap());
    }

    #[test]
    fn test_edge_case_large_heap() {
        let mut heap = MinHeap::new();

        for i in (0..1000).rev() {
            heap.insert(i as f64);
            assert!(heap.is_valid_heap());
        }

        for i in 0..1000 {
            assert_eq!(heap.delete(), Some(i as f64));
        }

        assert!(heap.is_empty());
    }
}
