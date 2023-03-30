use std::boxed::Box;
use std::mem;

/// The struct for the actual list element
pub struct List<T> {
    head: Link<T>
}

/// The enum type describing the link between a Node and its following value
type Link<T> = Option<Box<Node<T>>>;

/// The node that contains an element
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T>{
    /// Initializes a new list with an empty head
    pub fn new() -> Self {
        List {head: None}
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem, 
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    /// Remove an element from a list and give it here, Malfoy
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            Some(node.elem)
        }).unwrap()
    }

    /// Attempts to "peek" at an element in the linked list
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
    /// Peeks mutably
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
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