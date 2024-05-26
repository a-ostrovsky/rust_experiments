## Quickselect

Quickselect algorithm used to find k-th element in an unsorted sequence. 

### Steps:
1) A `pivot` element is selected from the sequence.
2) All elements that are less than the `pivot` are moved to its left.
3) If `k` is smaller than the index of the `pivot`, 
the algorithm continues to the left sub-array. 
If `k` is larger, it proceeds to the right sub-array. 
If `k` is equal to the index of the `pivot`, the pivot is returned as it is the kth smallest element.

### Complexity:
The average time complexity of Quickselect is _O(N)_, where _N_ is the size of the sequence. 
This might seem counterintuitive as one might expect it to be _O(N log(N))_. 
However, if we assume that at every iteration we select the middle element as the pivot, we look at N elements in the first iteration, N/2 in the second, N/4 in the third, and so on. This forms a geometric series: _O(N + N/2 + N/4 + N/8 + ...)_ which sums to _2N_, hence the overall complexity is _O(2N)_ which simplifies to _O(N)_.

In the worst-case scenario (when the smallest or largest element is always chosen as the pivot), the time complexity can degrade to _O(N^2)_.

### TODO:
* Add support for duplicate elements in the sequence.