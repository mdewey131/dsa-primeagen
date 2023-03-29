/// A simple function to define a sum based on a string of integers

pub fn sum_char_codes(text: &str) -> u32 {
    let mut sum = 0;
    for i in text.chars() {
        // Remember, rust doesn't let you just slice a string due to UTF-8 encoding 
        // Instead, since we're sure that this is going to contain ASCII characters,
        // we use as_bytes()
        let adder = i.to_digit(10).expect("Please only give my numbers");
        sum += adder;

    }
    return sum;
}