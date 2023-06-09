# Notes on Search
## Linear Search 
As a general note, it's good practive to visualize problems and attempt to discuss them. In other words, we're whiteboarding.

We'll be working with the simplest form of search, which is linear search over an array. 

Suppose we have an array from 0 to N. Out of the values contained, we wish to find the one that matches value v. In linear search,
we can go over the elements, figuring out if the value if the one we're looking for and, if so, returning that index. 

What is the Big O value for this type of search? It's O(N). In the worst case scenario, we have to search the entire array and don't find the value. The difficulty scales with the input linearly (e.g. an array of size 10 has twice the loops required compared to an array of size 5)

Here's an example of linear search, the simplest algorithm we can use
``` 
/// The function that implements linear search over an array
pub fn linear_search(haystack: &[u32], needle: u32) -> Option<usize> {
    let mut index_position: Option<usize> = None;
    for i in 0..haystack.len() {
        if haystack[i] == needle {
            index_position = Some(i);
            break;
        } 
    }
    return index_position;
}
```

With testing in the crate attached to this directory, it's easy to see that this algorithm does what we expect it to do.

## Binary Search
This one is a doozy, but it is a basis for a lot of other algorithms that we will encounter. 
Whenever you're working with a data set, you should ask yourself if its ordered. If it is, there are some really nice things you can do. 

If you have an ordered array, your search doesn't need to start at the 0th position. There are optimizations we could make. 
First, let's talk about what won't work (or at least, won't reduce the complexity of the algorithm):
1. Jump to the position at the next 10th percentile (starting with 10th) and check that value in the ordered array
1a. If this value is smaller than our needle continue
1b. If it's bigger, do a linear search over the 10 percentile range between the previous and the current

What's the big O value? Well, the total number of required searches in the worst case is 10 (for each percentile range) followed by a linear search over 10% of N. In other words,
the expression in terms of N is 10 + 0.1N, **which is still O(N)**. 

This is definitely a practical improvement over lienar search, just not a theoretical one. 

Binary search does something different. Instead of doing any linear scans (which automatically imply O(N) in the best case), we just look to see if the halfway point of an interval
is our value. This can continue until we find the value or correctly fail to find the value. 
This algorithm has a big O complexity of log N (specifically, log base 2 of N). This is because our search can stop when we've made enough cuts of the data, k, such that N / 2^k = 1 (remember, in the worst case). From this, it is easy to show that k would be equal to log N.

*Another helpful trick is revealed here: when doing algorithms that halve things, its usually O(log N) or O(N log N)*

### Implementing Binary Search
Here's the function, and I confirmed that this passes tests in the crate
```
pub fn binary_search(haystack: &[u32], needle: u32)  -> Option<usize> {
    let mut result: Option<usize> = None;
    let mut low: usize = 0;
    let mut high: usize = haystack.len();

    while low < high {
        let mid = (low + (high - low) / 2 as usize);
        if haystack[mid] == needle {
            result = Some(mid);
            break;
        } else if haystack[mid] < needle {
            // The value we're looking for is higher, let's move the low point up to the midpoint + 1
            low = mid + 1;
            continue;
        } else {
            // The value we're looking for is lower, so we move the highpoint down to the midpoint
            high = mid;
        }


    }
    return result;
}
```

## Two Crystal Balls
Here's the statement of the problem:
"Given two crystal balls that will break if dropped from a high enough distance, determine the exact spot in which it will break in the most optimized way"

Let's try and think through this problem. We can conceptualize an array of distances (ordered) in which everything that won't cause the ball to break is false, and every
distance (i.e. index) wherein it will break is true. Linearly walking through the array is too slow for this answer, but would conceptually represent the case of using
only one crystal ball and repeatedly dropping it until it breaks. 

Binary search wouldn't work either. Suppose you drop the ball on the halfway point and it breaks. Well, shit, now you have to linear search over the lower half, which is still O(N). 
So, can we do better? The answer is yes; we need to jump in such a way that we are not doing linear search. For example, let's say we jump the square root of N until our 
crystal ball breaks. Then, we will do linear search from the last good point to the bad point until our second ball breaks, at which point we've solved it. What's the running time of this algo?

It's O(sqrt(N)). This is because, in the worst case, we'd have to do a sqrt(N) jumps until our ball breaks, then do a search over an interval that is sqrt(N) across.

Here's a (not my favorite) implementation
```
pub fn two_crystal_balls(breaks: &[bool]) -> Option<usize> {
    let mut result = None;
    let len = breaks.len();
    // This coercion makes me feel dirty, but it seems to work okay
    let step_size = (len as f32).powf(0.5) as usize;
    let mut first_break = 0 as usize;
    // Use the first crystal ball to find the break 
    for i in (step_size..len).step_by(step_size) {
        if breaks[i] {
            first_break = i;
            result = Some(i);
            break;
        }
    }
    if first_break == 0 {
        return None;
    }
    // Then, use the second to find the first position at which the break occurs
    for j in (first_break - step_size)..first_break {
        if breaks[j] {
           result = Some(j);
        }
    } 
    return result;
}
```

