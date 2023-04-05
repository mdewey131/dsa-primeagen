use std::rc::Rc;
use std::cell::RefCell;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}


impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {head: None, tail: None}
    }

    pub fn push_front(&mut self, elem: T) {
        // New Node needs +2 links, everything else should be +0
        let new_head = Node::new(elem);

        match self.head.take() {
            Some(old_head) => {
                // Non-empty list, need to connect the old head
                old_head.borrow_mut().prev = Some(new_head.clone()); // +1 new head
                new_head.borrow_mut().next = Some(old_head);         // +1 old head
                self.head = Some(new_head);                          // +1 new head, -1 old head
            },
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // need to take the old head, ensuring that it's -2
        self.head.take().map(|old_head| {    // -1 old
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {          // -1 new
                    // not emptying list
                    new_head.borrow_mut().prev.take();             // -1 old
                    self.head = Some(new_head);                    // +1 new
                    // total: -2 old, +0 new
                }
                none => {
                    //emptying list
                    self.tail.take();                             // -1 old
                    // total: -2 old, no new
                }
            }
            // now, we want to return the old_head's inner element. but there's a problem. since 
            // this is inside an rc, which by definition doesn't like things being removed from it, we need
            // to try and get it out with `try_unwrap()`, which should give us the correct value since there is 
            // now only one strong reference to the old_head (since we've taken it out of the list structure)
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
}