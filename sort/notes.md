# Sorting!
## Bubble sort

Keep in mind the formal defintion of sorting: An array of size N is sorted if every constituent element x_i is such that x_i <= x_{i+1} for all 1 <= i < N

Bubble sort works by looking at each of these (i, i+1) pairs. By doing pairwise comparisons, we guarantee that the biggest element in the array will be at the end after the first iteration. 

Let's show this with the example array [1, 3, 7, 4, 2]. After one iteration of pairwise comparisons, we have [1, 3, 4, 2, 7]. Since we know that 7, the biggest element, is at the end, we don't have to include it in the next iteration. We get [1, 3, 2, 4, 7] after the next iteration. Proceeding on, [1, 2, 3, 4, 7] is the next value and we're done. 

So, we do N checks in the first case, N-1 in the second, N-2 in the third, and so on until we are looking at only N - N + 1 elements. 

Using the usual logic around addition of sequences such as this, there are N(N + 1) / 2 possible. You may think this goes to O(N^2 + N), but N is not considered a significant enough polynomial. So, bubble sort is O(N^2).

Okay, let's do this.

```
pub fn bubble_sort(array: &mut [u32]) {
    for i in 0..array.len() {
        for j in 0..(array.len() - i - 1) {
            if array[j] > array[j+1] {
                array.swap(j, j+1);
            }
        }
    }
}
```

Note that the actual function has a lot more commenting and documentation of the logic. 

There are more sorting strategies, but we won't be covering them right now

## Linked Lists
Problem with arrays
    1. Deletion of arrays is weird, and nulling out is not the same thing as deleting
    2. Insertion is not really possible, though writing is
    3. Arrays are ungrowable due to memory allocation constraints

So, let's talk linked lists
A linked list consists of nodes, which are elements that contain a value and the next node in the list
    A singly linked list contains only information about the next item in the list
    A doubly linked list contains information about both the previous and next items

One cool property about linked lists is that insertion and deletion can be incredibly fast. 
Suppose we have a list A <-> B <-> C <-> D and we want to add F inbetween A and B. We would then have to change F to point to A and B, for A to point to A, and for B to point to F. 
These operations, though they seem a bit strange, are effectively constant operations that can be used to change the order of the list. 

Let's pretend instead that we want to delete C. We have to break the link from B to C, setting B's next to D. Then, we also have to set D to refer back to B

The difficulty inherent in linked lists is that you have to do a lot of traversal and navigation to get to the correct place and access values. The nature of the structure means that order of operations is incredibly important. It's possible to accidentally drop references to elements of the list, effectively dropping everything inclduing and following the lost value. Remember, there's no index in a linked list
