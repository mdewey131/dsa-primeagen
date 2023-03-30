pub mod bubble_sort;
use bubble_sort::bubble_sort;


fn main() {
    let mut to_sort = [61, 3, 1, 2];
    println!("Unsorted: {:?}", to_sort);
    bubble_sort(&mut to_sort);
    println!("Sorted: {:?}", to_sort);

}
