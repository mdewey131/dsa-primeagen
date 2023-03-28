/// A simple function to define a sum based on a string of integers

pub fn sum_char_codes(n: &str) -> u8 {
    let mut sum = 0;
    for i in 0..n.len() {
        // Remember, rust doesn't let you just slice a string due to UTF-8 encoding 
        // Instead, since we're sure that this is going to contain ASCII characters,
        // we use as_bytes()
        let adder = n.as_bytes()[i];
        println!("pre sum: {sum}");
        sum += adder;
        println!("post sum: {sum}")

    }
    return sum;
}