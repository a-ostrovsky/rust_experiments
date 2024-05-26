use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;

pub type VertexRef<T> = Rc<RefCell<Vertex<T>>>;

pub struct Vertex<T> {
    value: T,
    neighbors: Vec<VertexRef<T>>,
}

impl<T> Vertex<T> {
    pub fn new(value: T) -> VertexRef<T> {
        Rc::new(RefCell::new(Vertex {
            value,
            neighbors: vec![],
        }))
    }

    pub fn add_edge(&mut self, neighbor: VertexRef<T>) {
        self.neighbors.push(Rc::clone(&neighbor));
    }
}

pub fn contains<T: Eq + Clone + Hash>(haystack: VertexRef<T>, needle: &T) -> bool {
    let mut visited = HashSet::new();
    let mut stack = vec![haystack];

    while let Some(current) = stack.pop() {
        let val = current.borrow();
        if &val.value == needle {
            return true;
        }
        if !visited.insert(val.value.clone()) {
            continue;
        }
        for neighbor in &val.neighbors {
            stack.push(Rc::clone(neighbor));
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn create_test_graph(depth: i32, ctr: &mut i32) -> VertexRef<i32> {
        *ctr += 1;
        let vertex = Vertex::new(*ctr);
        if depth > 0 {
            let mut borrowed = vertex.borrow_mut();
            borrowed.add_edge(create_test_graph(depth - 1, ctr));
            borrowed.add_edge(create_test_graph(depth - 1, ctr));
        }
        return vertex;
    }

    #[test]
    fn test_contains() {
        let mut ctr = 0;
        let root = create_test_graph(2, &mut ctr);
        // 3 levels -> 1 + 2 + 4 = 7 vertices
        assert!(contains(root.clone(), &7));
        assert!(!contains(root.clone(), &8));
    }
}
