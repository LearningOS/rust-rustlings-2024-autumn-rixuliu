/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM DONE

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // Start with a default value for the heap's root
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // Insert the new value at the end of the vector
        self.count += 1;
        if self.count < self.items.len() {
            self.items[self.count] = value;
        } else {
            self.items.push(value);
        }
        // Reheapify
        self.bubble_up(self.count);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_child = self.left_child_idx(idx);
        let right_child = self.right_child_idx(idx);

        if right_child <= self.count && (self.comparator)(&self.items[right_child], &self.items[left_child]) {
            right_child
        } else {
            left_child
        }
    }

    fn bubble_up(&mut self, idx: usize) {
        let mut current_idx = idx;

        while current_idx > 1 {
            let parent_idx = self.parent_idx(current_idx);
            if (self.comparator)(&self.items[current_idx], &self.items[parent_idx]) {
                // Swap if current is less than parent
                self.items.swap(current_idx, parent_idx);
                current_idx = parent_idx; // Move up
            } else {
                break; // No more bubbling up needed
            }
        }
    }

    fn bubble_down(&mut self, idx: usize) {
        let mut current_idx = idx;

        while self.children_present(current_idx) {
            let child_idx = self.smallest_child_idx(current_idx);
            if (self.comparator)(&self.items[child_idx], &self.items[current_idx]) {
                // Swap if child is less than current
                self.items.swap(current_idx, child_idx);
                current_idx = child_idx; // Move down
            } else {
                break; // No more bubbling down needed
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None; // No more elements to iterate
        }

        // Swap the first item (minimum/maximum) with the last item
        self.items.swap(1, self.count);
        let value = self.items.pop().unwrap(); // Remove the last item, which is the extracted value
        self.count -= 1;

        // Bubble down from the root to maintain the heap property
        if !self.is_empty() {
            self.bubble_down(1);
        }

        Some(value)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
