# Fenwick Tree Data Structure

This is an implementation of a Fenwick Tree in Rust. It is a data structure that supports dynamic prefix sum queries and updates on an array. This contrasts with prefix sums, which do not support dynamic updates efficiently.

## Complexities

- **Time**: Compute prefix sums and update an element in O(log n) time.
- **Space**: Requires O(n) space.

## How It Works

The Fenwick Tree uses a structure where each node (represented by an index) is responsible for a range of elements. This structure is based on the binary representation of indices:

1.  **Least Significant Bit (LSB)**: Each index `i` covers a range of elements determined by its least significant bit (LSB). The LSB can be calculated using `i & -i`.
2.  **Prefix Sum Query**: To compute the prefix sum up to index `i`, we sum the values at indices obtained by repeatedly subtracting the LSB from `i` until `i` becomes 0.
3.  **Update Operation**: To update the value at index `i`, we add the change to the values at indices obtained by repeatedly adding the LSB to `i` until `i` exceeds the array size.

**Note**: The Fenwick Tree operates directly on *indices*. If you are working with large or non-consecutive *values* (e.g., 999, 2000, 2433, 9999), you might need to map these values to a smaller, consecutive range of indices (0 to k-1, where k is the number of distinct values) before using the tree.

### Example

Consider an array of size 16 (indices 1 to 16). The binary representation of indices determines the range of elements each index is responsible for:

- Index 12 (1100 in binary) holds the children 13, 14, and 15 (1101, 1110, and 1111 in binary).
- Index 8 (1000 in binary) holds the children 9, 10, 11, 12, 13, 14, and 15.

### Step-by-Step Operations

#### Prefix Sum Query
To compute the prefix sum up to index `i`:
1. Start with `sum = 0`.
2. Add the value at index `i` to `sum`.
3. Update `i` to `i - (i & -i)`.
4. Repeat until `i` becomes 0.

#### Update Operation
To update the value at index `i`:
1. Add the difference to the value at index `i`.
2. Update `i` to `i + (i & -i)`.
3. Repeat until `i` exceeds the size of the array.