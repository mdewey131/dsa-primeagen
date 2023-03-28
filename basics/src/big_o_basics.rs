/// This function takes a string of integers and produces the sum of those integers
pub fn char_to_sum(string: &str) -> u32 {
    let mut sum = 0;
    for i in string.chars() {
        sum += i.to_digit(10).expect("Passed garbage");
    }
    return sum;
}