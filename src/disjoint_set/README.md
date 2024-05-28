# Disjoint Set Data Structure

An implementation of a disjoint set. This data structure keeps track of a partition of a set into disjoint, non-overlapping subsets. It supports three main operations:

- `make_set(x)`: Creates a new set containing the element `x`. Initially, `x` is the  parent of this set consisting of one element.

- `find(x)`: Find the representative element of the set containing `x`. This operation uses path halving to keep the tree flat. Possible alternatives to path halving are path splitting (replacing node to parent by node to grandparent) and finding the root and updating nodes parents on the path.

- `union(x, y)`: Merge the sets containing `x` and `y` into a single set.

The `union by rank` strategy is used to maintain balance in the tree structure. Each element is associated with a rank, which serves as an upper bound for the height of the tree measured starting from root to the leave. It is not the height itself due to path compression which doesn't change the rank.
When two sets are merged, the following scenarios may occur:

- `rank(x) < rank(y)`: `x` becomes a child of `y`. This operation does not affect the upper bound of the height of `y`.
- `rank(x) > rank(y)`: `y` becomes a child of `x`. Similar to the previous operation, the upper bound of the height of `x` remains unaffected.
- `rank(x) == rank(y)`: `x` becomes a child of `y` and the rank of `y` is incremented by 1. This is because the height of the tree could potentially increase. (Also 'y' can become child of `x` while incrementing the rank of 'x')