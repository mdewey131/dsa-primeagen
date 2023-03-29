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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_found_match() {
        let array: [u32; 6] = [0, 1, 2, 3, 4, 5];
        assert_eq!(linear_search(&array, 5), Some(5));
    }

    #[test]
    fn test_found_no_match() {
        let array: [u32; 6] = [0, 1, 2, 3, 4, 5];
        assert_eq!(linear_search(&array, 7), None);
    }
}



