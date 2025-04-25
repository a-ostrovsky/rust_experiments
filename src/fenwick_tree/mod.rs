pub struct FenwickTree<T> {
    data: Vec<T>,
}

impl<T: std::ops::AddAssign + Copy + Default> FenwickTree<T> {
    pub fn new(size: usize) -> Self {
        FenwickTree {
            data: vec![T::default(); size + 1],
        }
    }

    pub fn from_slice(data: &[T]) -> Self {
        let mut fenwick_tree = FenwickTree::new(data.len() - 1);
        for i in 1..data.len() {
            fenwick_tree.add(i, data[i]);
        }
        fenwick_tree
    }

    pub fn add(&mut self, index: usize, value: T) {
        let mut i = index;
        while i < self.data.len() {
            self.data[i] += value;
            i += i & i.wrapping_neg();
        }
    }

    pub fn query(&self, index: usize) -> T {
        let mut sum = T::default();
        let mut i = index;
        while i > 0 {
            sum += self.data[i];
            i -= i & i.wrapping_neg();
        }
        sum
    }

    pub fn range_query(&self, left: usize, right: usize) -> T
    where
        T: std::ops::Sub<Output = T>,
    {
        if left > right {
            panic!("Left index cannot be greater than right index")
        }
        self.query(right) - self.query(left - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_fenwick_tree() {
        let fenwick_tree: FenwickTree<i32> = FenwickTree::new(0);
        assert_eq!(fenwick_tree.query(0), 0);
    }

    #[test]
    fn test_fenwick_tree_distinct_elements() {
        let data = vec![0, 1, 2, 3, 4, 5];
        let mut fenwick_tree = FenwickTree::from_slice(&data);
        assert_eq!(fenwick_tree.query(3), 6);
        assert_eq!(fenwick_tree.range_query(1, 3), 6);
        fenwick_tree.add(2, 3);
        assert_eq!(fenwick_tree.query(3), 9);
        assert_eq!(fenwick_tree.range_query(1, 3), 9);
    }

    #[test]
    fn test_fenwick_tree_same_elements() {
        let data = vec![0, 1, 1, 1, 1, 1];
        let mut fenwick_tree = FenwickTree::from_slice(&data);
        assert_eq!(fenwick_tree.query(3), 3);
        assert_eq!(fenwick_tree.range_query(1, 3), 3);
        fenwick_tree.add(2, 3);
        assert_eq!(fenwick_tree.query(3), 6);
        assert_eq!(fenwick_tree.range_query(1, 3), 6);
    }
}
