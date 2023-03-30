/// Implements bubble sort with a mutable array reference
pub fn bubble_sort(mut array: [u32]) -> [u32] {
    // We need to iterate over each element of the array in the first loop except the first. -- i in 0..n - 1
    // In the second, we iterate over every element except for the last two    -- i in 0..n-2
    // In the third, we iterate over every element except for the last three -- i in 0..n-3
    // ...
    // In the n-1th, we iterate over every element except for the last n-1    -- i in 0..n-(n-1) = 0..1
    // And we're done at n-2, because that loop is just 0
    for i in 0..array.len() - 1 {
        for j in 0..(array.len() - i) {
            if array[j] > array[j+1] {
                let tmp = array[j];
                array[j] = array[j+1];
                array[j+1] = tmp;
            }
        }
    }
    return array;
}