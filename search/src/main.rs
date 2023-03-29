pub mod linear_search;
pub mod binary_search;
pub mod two_crystal_balls;
use linear_search::*;
use binary_search::*;
use two_crystal_balls::*;
fn main() {
    let array: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result = binary_search(&array, 1);
    println!("Result: {:?}", result);
}
