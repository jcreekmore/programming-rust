// Binary heap algorithm taken from https://en.wikipedia.org/wiki/Binary_heap

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct MinBinaryHeap<T: Debug + Ord> {
    buf: Vec<T>,
}

impl<T: Debug + Ord> MinBinaryHeap<T> {
    pub fn new() -> MinBinaryHeap<T> {
        MinBinaryHeap { buf: Vec::new() }
    }

    fn last_idx(&self) -> usize {
        self.buf.len() - 1
    }

    fn left(&self, idx: usize) -> Option<usize> {
        let new = (2 * idx) + 1;
        if new <= self.last_idx() {
            Some(new)
        } else {
            None
        }
    }

    fn right(&self, idx: usize) -> Option<usize> {
        let new = (2 * idx) + 2;
        if new <= self.last_idx() {
            Some(new)
        } else {
            None
        }
    }

    fn parent(&self, idx: usize) -> usize {
        assert!(idx > 0);
        (idx - 1) / 2
    }

    fn trivial(&self) -> bool {
        self.buf.len() == 1
    }

    pub fn root(&self) -> Option<&T> {
        self.buf.get(0)
    }

    /// Insert a value into the heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use binheap::MinBinaryHeap;
    /// let mut heap = MinBinaryHeap::new();
    /// heap.insert(42);
    /// ```
    ///
    /// ```
    /// use binheap::MinBinaryHeap;
    /// let mut heap = MinBinaryHeap::new();
    /// heap.insert("Foo Bar");
    /// ```
    pub fn insert(&mut self, value: T) {
        // 1. Add the element to the bottom level of the heap
        self.buf.push(value);

        // 1a. if it is a trivial heap, return early
        if self.trivial() {
            return;
        }

        let mut idx = self.last_idx();
        loop {
            let parent = self.parent(idx);

            // 2. Compare the added element with its parent; if they are in the correct order, stop.
            if self.buf[parent] <= self.buf[idx] {
                return;
            }

            // 3. If not, swap the element with its parent and return to the previous step.
            self.buf.swap(parent, idx);
            idx = parent;

            // 3a. if we swap into the root element, we are done
            if idx == 0 {
                return;
            }
        }
    }

    /// Extract the minimum value from the heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use binheap::MinBinaryHeap;
    /// let mut heap = MinBinaryHeap::new();
    /// for x in vec![42, 5, 13, 2] {
    ///     heap.insert(x);
    /// }
    /// let min = heap.extract().unwrap();
    /// ```
    pub fn extract(&mut self) -> Option<T> {
        // if the buffer is empty, we cannot extract anything
        if self.buf.is_empty() {
            return None;
        }

        // 1. Replace the root of the heap with the last element on the last level.
        let value = self.buf.swap_remove(0);

        // 1a. if we have a trivial heap left, just return the value
        if self.buf.is_empty() || self.trivial() {
            return Some(value);
        }

        let mut idx = 0;
        loop {
            let left = self.left(idx);
            let right = self.right(idx);

            {
                let current = &self.buf[idx];

                // 2. Compare the new root with its children;
                // if they are in the correct order, stop.
                match (left, right) {
                    (Some(left), Some(right)) if current <= &self.buf[left] &&
                                                 current <= &self.buf[right] => {
                        return Some(value);
                    }
                    (Some(left), None) if current <= &self.buf[left] => {
                        return Some(value);
                    }
                    (None, None) => {
                        return Some(value);
                    }
                    (_, _) => (),
                };
            }

            // 3. If not, swap the element with one of its children and
            // return to the previous step.
            let smallest = match (left, right) {
                (Some(left), Some(right)) => {
                    if self.buf[left] < self.buf[right] {
                        left
                    } else {
                        right
                    }
                }
                (Some(left), None) => left,
                (_, _) => unreachable!(),
            };
            self.buf.swap(idx, smallest);
            idx = smallest;
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};
    use super::*;

    impl<T: Arbitrary + Debug + Ord> Arbitrary for MinBinaryHeap<T> {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let mut src: Vec<T> = Vec::arbitrary(g);
            let mut arb = Self::new();

            for value in src.drain(..) {
                arb.insert(value);
            }

            arb
        }
    }

    quickcheck! {
        fn prop_root_is_minimum_value(xs: MinBinaryHeap<usize>) -> bool {
            let min = xs.buf.iter().min();
            match (min, xs.root()) {
                (Some(min), Some(root)) => min == root,
                (_, None) => true,
                (None, _) => unreachable!(),
            }
        }

        fn prop_is_heap(xs: MinBinaryHeap<usize>) -> bool {
            for idx in 0..xs.buf.len() {
                let current = &xs.buf[idx];

                // If there is a right child
                if let Some(right) = xs.right(idx) {
                    // It must be smaller than the current value
                    if current > &xs.buf[right] {
                        return false;
                    }
                }

                // If there is a left child
                if let Some(left) = xs.left(idx) {
                    // It must be smaller than the current value
                    if current > &xs.buf[left] {
                        return false;
                    }
                }
            }
            true
        }

        fn prop_successive_extracted_values_are_greater(xs: MinBinaryHeap<usize>) -> bool {
            let mut xs = xs;
            let mut prev: Option<usize> = None;

            loop {
                match (prev, xs.extract()) {
                    (Some(old), Some(new)) if old > new => { return false; },
                    (Some(_), Some(new)) => { prev = Some(new); },
                    (None, Some(new)) => { prev = Some(new); },
                    (_, None) => { return true; },
                }
            }
        }
    }

    #[test]
    fn new_heap_empty() {
        let mut heap: MinBinaryHeap<usize> = MinBinaryHeap::new();
        assert_eq!(heap.buf.len(), 0);
        assert_eq!(heap.extract(), None);
    }

    #[test]
    #[should_panic]
    fn new_heap_empty_unwrap() {
        let mut heap: MinBinaryHeap<usize> = MinBinaryHeap::new();
        heap.extract().unwrap();
    }

    #[test]
    fn insert_adds_value() {
        let mut heap = MinBinaryHeap::new();
        heap.insert(42);
        assert_eq!(*heap.root().unwrap(), 42);
    }

    #[test]
    fn extract_pulls_min_value() {
        let mut heap = MinBinaryHeap::new();
        for x in vec![42, 5, 13, 2] {
            heap.insert(x);
        }
        let value = heap.extract().unwrap();
        assert_eq!(value, 2);
    }
}
