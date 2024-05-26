## Graph Implementations

Two different implementations of a graph data structure in Rust: `rc_graph.rs` and `arena_graph.rs`.

### `rc_graph.rs`
This file contains an implementation of a graph using `Rc` and `RefCell` for shared and mutable ownership. This allows to avoid unsafe code albeit at the cost of overhead associated with a reference counter.

### `arena_graph.rs`
This file contains an implementation of a graph using an arena allocator. This requires unsafe code due
to possible circular references.

### TODO
* Write a better text here.
* Benchmark.