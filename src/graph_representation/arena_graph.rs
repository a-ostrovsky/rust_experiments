use std::hash::Hash;
use std::{cell::UnsafeCell, collections::HashSet};
use typed_arena::Arena;

pub struct Vertex<'a, T> {
    value: T,
    neighbors: UnsafeCell<Vec<&'a Vertex<'a, T>>>,
}

impl<'a, T> Vertex<'a, T> {
    pub fn new(value: T, arena: &'a Arena<Vertex<'a, T>>) -> &'a mut Self {
        arena.alloc(Vertex {
            value,
            neighbors: UnsafeCell::new(Vec::new()),
        })
    }

    pub fn get_neighbors(&self) -> &mut Vec<&'a Vertex<'a, T>> {
        unsafe { &mut *self.neighbors.get() }
    }

    pub fn add_edge(&mut self, neighbor: &'a Vertex<'a, T>) {
        unsafe { (*self.neighbors.get()).push(neighbor) }
    }
}

pub fn contains<T: Eq + Clone + Hash>(haystack: &Vertex<T>, needle: &T) -> bool {
    let mut visited = HashSet::new();
    let mut stack = vec![haystack];

    while let Some(current) = stack.pop() {
        if &current.value == needle {
            return true;
        }
        if !visited.insert(current as *const _) {
            continue;
        }
        for neighbor in current.get_neighbors() {
            stack.push(neighbor);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn create_test_graph<'a>(
        arena: &'a Arena<Vertex<'a, i32>>,
        depth: i32,
        ctr: &mut i32,
    ) -> &'a Vertex<'a, i32> {
        *ctr += 1;
        let vertex = Vertex::new(*ctr, arena);
        if depth > 0 {
            vertex.add_edge(create_test_graph(arena, depth - 1, ctr));
            vertex.add_edge(create_test_graph(arena, depth - 1, ctr));
        }
        return vertex;
    }

    #[test]
    fn test_contains() {
        let arena = Arena::new();
        let mut ctr = 0;
        let root = create_test_graph(&arena, 2, &mut ctr);
        // 3 levels -> 1 + 2 + 4 = 7 vertices
        assert!(contains(root, &7));
        assert!(!contains(root, &8));
    }
}
