/// The function that implements binary search
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_returns_some() {
        let array = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&array, 4), Some(3));
    }

    #[test]
    fn correctly_returns_none() {
        let array = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&array, 0), None);
    }
}