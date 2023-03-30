use std::{Rc, Box};
/// My implementation of a doubly linked list in the basic box way
/// Rc has to be used here because, in a doubly linked list, there
/// need to be multiple pointers to the same data to handle
/// ownership issues
pub struct DoubleNode {
    value: u32,
    next: Option<Rc<Box<DoubleNode>>>,
    previous: Option<Rc<Box<DoubleNode>>>
}