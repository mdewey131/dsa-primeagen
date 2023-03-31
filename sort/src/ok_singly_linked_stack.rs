use std::boxed::Box;
use std::mem;

/// A thin wrapper around the list for iteration
pub struct IntoIter<T>(List<T>);

/// A struc that takes references to list nodes for iteration. Does not consume the list
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

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
        }).unwrap_or_else(|| None)
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
     
    /// Converts the List into an IntoIter class for iteration
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    
    /// Creates a Iter object looking at the next element of the List
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        // as_deref() allows us to move out of the box and get the node that's being expected
        Iter { next: self.head.as_deref() }
    }
}

impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
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
    
    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| {
            *value = 42
        });
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}