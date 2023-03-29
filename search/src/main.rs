pub mod linear_search;
use linear_search::*;
fn main() {
    let array: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result = linear_search(&array, 11);
    println!("Result: {:?}", result);
}
