/// Implements bubble sort with a mutable array reference
/// This has to be done with a mutable reference to the array,
/// because rust needs a fixed size to allocate on the stack 
pub fn bubble_sort(array: &mut [u32]) {
    // We need to iterate over each element of the array in the first loop except the first. -- i in 0..n - 1
    // In the second, we iterate over every element except for the last two    -- i in 0..n-2
    // In the third, we iterate over every element except for the last three -- i in 0..n-3
    // ...
    // In the n-1th, we iterate over every element except for the last n-1    -- i in 0..n-(n-1) = 0..1
    // And we're done at n-1, because that loop executes just once
    for i in 0..array.len() {
        for j in 0..(array.len() - i - 1) {
            if array[j] > array[j+1] {
                array.swap(j, j+1);
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sorts_array() {
        let mut array = [7, 5, 6, 1, 8, 3];
        bubble_sort(&mut array);
        assert_eq!(array, [1, 3, 5, 6, 7, 8]);
    }
}