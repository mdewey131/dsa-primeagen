/// An implementation of two crystal balls
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn correctly_find_break() {
        let array = [false, false, false, true, true, true, true, true, true];
        assert_eq!(two_crystal_balls(&array), Some(3));
    }
    
    #[test]
    fn correctly_find_no_break() {
        let array = [false, false, false, false];
        assert_eq!(two_crystal_balls(&array), None);
    }
}