use std::boxed::Box;
use std::mem;

/// The struct for the actual list element
pub struct List {
    head: Link
}

/// The enum type describing the link between a Node and its following value
type Link = Option<Box<Node>>;

/// The node that contains an element
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    /// Initializes a new list with an empty head
    pub fn new() -> Self {
        List {head: Link::Empty}
    }

    pub fn push(&mut self, elem: i32) {
        // Use of replace allows us to change the value inside of the mutable reference without triggering errors from moving out of the reference
        // This is really useful because Rust always expects us to keep the mutable reference "intact" insofar as there must be *something* valid there
        let new_node = Box::new(Node {
            elem: elem, 
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Some(new_node);
    }

    /// Remove an element from a list and give it here, Malfoy
    pub fn pop(&mut self) -> Option<i32> {
        // Same reasons as before, we have to do the dance here
        match mem::replace(&mut self.head, Link::Empty) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed node then goes out of scope and gets dropped here
            // however, it's Node's "next" field has been set to Link::Empty, so no unbounded 
            // recusion occurs within this while loop

        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        // Populate the list
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));

        assert_eq!(list.pop(), None);

        list.push(4);
        list.push(5);
        
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
    }
    
}