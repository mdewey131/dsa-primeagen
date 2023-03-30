/// The first attempt at a linked list
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}