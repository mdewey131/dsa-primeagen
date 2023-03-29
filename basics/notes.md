# Basics
## Introduction Section
Mostly intro to the author
## Recommended books
- The Introduction to Algorithms
- A Common Sense Guide to Data Structures and Algorithms
- (This one I've personally come across, not in course) The Algorithm Design Manual by Skiena

# Okay, The Actual Basics Section
## Big O
Big O is a way to categorize your algorithm's time or memory requirements based on inputs. It is not meant to be an exact measurement. It will not tell you how many CPU cycles something takes, for example, but generalizes the idea of your algo's growth. 
For example, an O(N) algo grows linearly based on input

Big O is useful to help us make decisinos about what data structures and algorithms we should use. 
Let's do a small example.

``` 
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
```