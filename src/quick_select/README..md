## Quickselect

Quickselect algorithm used to find k-th element in an unsorted sequence. 

### Steps:
1) Select a `pivot` element.
2) Move all elements which are less than the `pivot` to the left of it.
3) If `k` is smaller than the index of `pivot`, then search in the left sub-array.
If it is bigger, then search in the right sub-array. Otherwise return the pivot.

### Complexity:
In average the complexity is _O(N)_ while in the worst case it can be _O(N^2)_ with _N_ being the size of the sequence.
The complexity is not _O(N log(N))_ as one might think. 
Assume that at every iteration we magically select the middle element as pivot element.
Now, in the first iteration we look at N elements, then at N/2, then at N/4, etc. So the complexity is
_O(N + N/2 + N/4 + N/8 + ...)_. This is the geometric series converting to _2N_.
The overall complexity is _O(2N)_ = _O(N)_

### TODO:
* Support duplicates
* Better selection of pivot