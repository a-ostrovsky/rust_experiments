use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct DisjointSet<T: Hash> {
    parent: HashMap<T, T>,
    rank: HashMap<T, u8>,
}

impl<T: Hash + Eq + Copy> DisjointSet<T> {
    pub fn new() -> Self {
        Self {
            parent: HashMap::new(),
            rank: HashMap::new(),
        }
    }

    pub fn make_set(&mut self, x: T) {
        self.parent.insert(x, x);
        self.rank.insert(x, 0);
    }

    pub fn find(&mut self, x: T) -> Option<T> {
        let mut node = x;
        if !self.parent.contains_key(&x) {
            return None;
        }
        while self.parent.get(&node) != Some(&node) {
            let parent = *self.parent.get(&node).unwrap();
            let grandparent = self.parent.get(&parent).unwrap();
            self.parent.insert(node, *grandparent);
            node = parent;
        }
        Some(node)
    }

    pub fn union(&mut self, a: T, b: T) {
        let a_parent = self.find(a).unwrap();
        let b_parent = self.find(b).unwrap();
        if a_parent == b_parent {
            return;
        }
        if self.rank.get(&a_parent) < self.rank.get(&b_parent) {
            self.parent.insert(a_parent, b_parent);
        } else {
            self.parent.insert(b_parent, a_parent);
            if self.rank.get(&a_parent) == self.rank.get(&b_parent) {
                *self.rank.get_mut(&b_parent).unwrap() += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disjoint_set() {
        let mut ds = DisjointSet::new();
        ds.make_set(1);
        ds.make_set(2);
        ds.make_set(3);
        ds.make_set(4);
        ds.make_set(5);
        ds.make_set(6);
        ds.make_set(7);

        // First set
        ds.union(1, 2);
        ds.union(2, 3);

        // Second set
        ds.union(4, 5);
        ds.union(6, 7);
        ds.union(5, 6);

        assert_eq!(ds.find(1), ds.find(3));
        assert_eq!(ds.find(4), ds.find(7));
        assert_ne!(ds.find(1), ds.find(4));
    }
}
