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

### Understanding Big O through a small example
Let's take the following example. What would this function's Big O notation be?
``` 
pub fn sum_char_codes(text: &str) -> u32 {
    let mut sum = 0;
    for i in text.chars() {
        // Remember, rust doesn't let you just slice a string due to UTF-8 encoding. Instead,
        // we take the chars and trust that the expect will work because we won't give ourselves
        // non digit entries. This should be properly checked in an actual function
        let adder = i.to_digit(10).expect("Please only give my numbers");
        sum += adder;

    }
    return sum;
}
```

Remember: Growth in Big O is with respect to the input

So, how does sum_char_code's function increase with respect to input? Linearly. For each additional value in the string, there is one additional loop to perform. That's plainly O(n).
The easiest trick to grok big O is to look at the number of loops.

So, if that was O(n), what would this be?
```
pub fn sum_char_codes_two_loops(text: &str) -> u32 {
    let mut sum = 0;
    for i in text.chars() {
        let adder = i.to_digit(10).expect("Please only give me numbers");
        sum += adder;

    }

    for i in text.chars() {
        let adder = i.to_digit(10).expect("Please only give me numbers");
    }

    return sum;
}
```

In big O, we actually drop constants, because things affine changes to to runtime of the algorithm is "not important" relative to even tiny exponential changes.

### Practical vs Theoretical Differences
Just because N is faster than N^2 in theoretical terms, doesn't mean that N is faster in every practical case. The key is that, if data is small, the constant that is dropped will be bigger than the exponentiated term.

### Another Example
What is the running time of this algorithm?
```
pub fn sum_char_codes_example_3(text: &str) -> u32 {
    let mut sum = 0;
    for i in text.chars() {
        let adder = i.to_digit(10);
        match adder {
            Some(to_add) => {sum += to_add;},
            None => {
                return sum;
            }
        }
    }
    return sum;
}
```

Notice that this still has a potential early exit condition. However, that does not affect the value that we ascribe to the algorithm. The important concept here is that the worst case is the one that we measure. 


### Common Complexities
 - O(1)
 - O(log n)
 - O(n)
 - O(n log n)
 - O(n ^ 2)
 - O(2 ^ n)
 - O(n!)

 The last two of these are basically impossible with modern computers.

 #### Some Examples
 O(n^2)

 ``` 
pub fn sum_char_codes_O_n2(text: &str) -> u32 {
    let mut sum = 0;
    for i in text.chars() {
        for _j in text.chars() {
            let adder = i.to_digit(10);
            match adder {
                Some(to_add) => {sum += to_add;},
                None => { return sum;}
            };
        }
    }
    return sum;
}
```


