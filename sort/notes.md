# Sorting!
## Bubble sort

Keep in mind the formal defintion of sorting: An array of size N is sorted if every constituent element x_i is such that x_i <= x_{i+1} for all 1 <= i < N

Bubble sort works by looking at each of these (i, i+1) pairs. By doing pairwise comparisons, we guarantee that the biggest element in the array will be at the end after the first iteration. 

Let's show this with the example array [1, 3, 7, 4, 2]. After one iteration of pairwise comparisons, we have [1, 3, 4, 2, 7]. Since we know that 7, the biggest element, is at the end, we don't have to include it in the next iteration. We get [1, 3, 2, 4, 7] after the next iteration. Proceeding on, [1, 2, 3, 4, 7] is the next value and we're done. 

So, we do N checks in the first case, N-1 in the second, N-2 in the third, and so on until we are looking at only N - N + 1 elements. 

Using the usual logic around addition of sequences such as this, there are N(N + 1) / 2 possible. You may think this goes to O(N^2 + N), but N is not considered a significant enough polynomial. So, bubble sort is O(N^2).

Okay, let's do this.