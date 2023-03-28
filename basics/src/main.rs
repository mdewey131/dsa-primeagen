mod basic_big_o;
use basic_big_o::*;

fn main() {
    let sum = sum_char_codes("0123".into());
    println!("Sum of numbers: {sum}");
}
