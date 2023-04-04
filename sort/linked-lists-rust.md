# Making Linked Lists in Rust
Because we're looking at the linked lists portion of the DSA Course, I'm also going to bring in the [Rust book on learning linked lists](https://rust-unofficial.github.io/too-many-lists/). I don't think this is a necessary detour, but it will be a way for me to build on all of the stuff I've been learning. As such, this markdown file exists to detail my process of going through that book at a cursory level, learning the trappings of different types of linked lists in Rust.

It is probably worth mentioning that the author of the book linked generally recommends against using linked lists in rust. There are other data structures which emulate the functionality. The rust documentation on the LinkedList data structure also mentions that a Vec or VecDeque is a better solution in Rust because "array-based containers are generally faster, more memory efficient, and make better use of the CPU cache."

## Pros and Cons of Linked Lists
Linked lists, as mentioned in the DSA course, perform their operations to insert, append, and remove in constant time. However, this book argues that such an approach is not really worth it if the time it takes to access some object deep in the list is longer than the time it takes to simply copy and reallocate,. Linked lists, then, are a good option if profiling reveals a lot of time spent splitting and merging list elements

The memory savings that are possible really depends on the object size too. "If you have huge elements, can't predict the load, and have a decent allocator, there are memory savings to be had"

Linked lists are the heroes of the "dark world of lock-free concurrency", but we're unlikely to ever need to build something like that.

# 1. A Bad Singly-Linked Stack
## A first attempt
In the world of functional programming, a linked list would be defined as "either Empty, or an element followed by a list". This definition is inherently recursive and, since there are different types, our translation into the Rust world naturally uses enums! Let's transcribe the functional definition into rust. Though this can be made generic, we'll only work with signed-32 bit integers for the time being.
```
pub enum List {
    Empty,
    Elem(i32, List),
}
```

The problem with this definition in Rust is that List has a potentially infinite size, which causes compile-time errors. To fix this, we have to use a Box, which gives our List a fixed size that can be allocated properly. So, instead, we can wrap the list inside of `Elem` with a Box.
```
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}
```

This definition is okay, but suffers from a few different issues. Consider a list with two elements, which would have to be constructed as follows:
Let [] refer to a stack allocation, and () a heap allocation
[Elem(A, ptr)] -> (Elem(B, ptr)) -> (Elem(Empty, *junk*))
Notice that we have a las element, which is just some piece of data that we allocated but isn't even a Node at all (in fact, it exists just to tell us it isn't a node). Moreover, in the way this would actually be defined, Elem A would be stack allocated while B and the junk element would be *heap* allocated. These two almost "cancel each other out". Although we've heap allocated an extra node, one of our nodes doesn't need to be heap-allocated at all. 

## Fixing the layout
Let's consider a different layout
[ptr] -> (Elem(A, ptr)) -> (Elem(B, *null*))

In this layout, our nodes are unconditionally heap-allocated, and we've gotten rid of the junk. This is really important because Enum allocation works off of the largest element, so even though you could think about `Empty` as being something quite small (indeed, even a single bit), its actual allocation footprint is as big as Elem's. Layout 1 also suffers from problems related to moving elements about, effectively nullifying one of the best properties of linked lists - their ease of movement

Let's rewrite the list to try and solve this problem, keeping in mind that there are now two essential objects we wish to speak about: a List, and a Node
```
/// The node that contains an element
struct Node {
    elem: i32,
    next: List,
}
/// The list that holds the node objects
pub enum List {
    Empty,
    More(Box<Node>),
}
```

Checking our requirements from before:
- The tail of the list should never allocate extra junk: check!
- enum is in a null-pointer optimized form (meaning, there are two states, one of which is all "non-nulls" and another of which can be represented with all 0's): check!
- All elements are uniformly allocated: check!

This above implementation has problems with public and private. In particular, we're leaking a private type `Node` in the public interface of a List. That's a problem. Instead, let's create a structure where we can hide the private implementation details
```
/// The struct for the actual list element
pub struct List {
    head: Link
}

/// The enum type describing the link between a Node and its following value
enum Link {
    Empty,
    More(Box<Node>),
}

/// The node that contains an element
struct Node {
    elem: i32,
    next: Link,
}
```

Because the `List` is a struct with a single field, its size is the same as the field (hello zero-cost abstraction).

## Making this actually useful
We've managed to do nothing with the lists we're actually building! Let's fix that with impl blocks
```
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

        self.head = Link::More(new_node);
    }
```
Pay special attention to how push works. In this instance, we take a mutable reference to the List that we're working with, and add a node with the element. However, we have to use the function "replace" so that the mutable reference stays intact. This is basically the ol` Indiana Jones swap, since `replace` returns the value that was inside. We're literally taking the pointer from the list's head, putting a temporary empty value, then overwriting that value with the Node we've just created. This is not exactly optimal, but it works for now.

Next, let's explore a pop function to mutate the list. Unlike push, this needs to return something (viz. the element that we're popping), but it has a corner case because lists are empty sometimes. Therefore, this return type has to be an option
```
/// Remove an element from a list and give it here, Malfoy
    pub fn pop(&mut self) -> Option<i32> {
        // Same reasons as before, we have to do the dance here
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
```
Notice that we, again, have to do the same dance of replacing a value. That's fine in this case, because we're either replacing with the same thing, which gives us the None value as intended, or we're using the value with the Link::More(node) piece

## Implementing Drop, and why we should
You don't strictly need to implement Drop here. All you'd need to do is let a thing go out of scope, and Rust will handle it. However, there's a problem in this case: the automatic handling of this is going to be bad. 

Consider the following list

list -> A -> B -> C

The problem is that dropping A means you can no longer drop B! You've lost the pointer to the box containing the memory. That's an issue. Rust is also not smart enough to handle dropping in a tail recursive fashion, at least not automatically. So, instead, we have to do some work to implement the trait ourselves.
```
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed node then goes out of scope and gets dropped here
            // however, it's Node's "next" field has been set to Link::Empty, so no unbounded 
            // recusion occurs within this while loop

        }
    }
}
```

# 2. An Ok Singly-Linked Stack
We're going to now attempt to make a different kind of singly linked stack, and make it less sucky.

First, our use of Link from before is basically just a reinvention of Option. That's not particularly good, so we can improve that using aliases!
```
pub struct List {
    head: Link
}

type Link = Option<Box<Node>>
```
Note that all the methods that used Link before need to be reworked to use the option. This is reflected in the actual script, but not replicated here. Most importantly, this allows us to use stuff that Option has built in. Take `take` which is actually a method that replicates `mem::replace(&mut option, None)`. We can also use map() to simplify some of the match statements that are essentially None -> None and Some(x) -> Some(y). Finally, we're going to make the entire thing generic

### Adding New Functionality with Peek
We're going to now try to peek at an element, just attempting to see the value that comes out from the head of the list

``` 
pub fn peek(&self) -> Option<&T> {
    self.head.map(|node| {
        &node.elem
    })
}
```

### Implementing IntoIter, Iter, and IterMut
#### IntoIter
Again, in the interest of adding functionality, we'll include the Iterator trait on the List we're working with. This has an associated type to know what it is supposed to put out from the use of the iterator. Interestingly, this is accomplished using a shallow struct wrapper over the list, and then implementing Iterator on that struct, as below:

```
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
```

Notice that this is moving the ownership of the List into the iterator, in other words, consuming the thing we're iterating over.

#### Iter
In this case, we're not coercing the list into an iterator and consuming it, instead we're taking a pointer to each of the nodes, which can either exist or not (either because the list is empty or because we're on the last item of iteration). So, we'll have to use an Option. 

In fact, we still use the thin wrapper, but we also have to introduce lifetimes. Because we're trying to take a reference at every step of the iterator, we have to guarantee to the compiler that the list which is being referenced has to live at least as long as the iterator taking a reference from the list. That leads to the following type definitions and implentations:
```
/// A struc that takes references to list nodes for iteration. Does not consume the list
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}
impl<T> for List<T> {
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
```
### IterMut
This one is, apparently, WILD. Which scares me. 

Semantically, implementing Iter and IterMut is fairly similar, but these go through dark magics due to requiring mutable references, which are not allowed to coexist.

Our first pass will just change stuff from immutable to mutable. 
```
/// A struc that takes references to list nodes for iteration. Does not consume the list
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}
impl<T> for List<T> {
    /// Creates a IterMut object looking at the next element of the List
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

   fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}
```
Here is the error we get if we run this
```
cannot move out of `self.next` which is behind a mutable reference
move occurs because `self.next` has type `Option<&mut Node<T>>`, which does not implement the `Copy` trait
``` 

This error actually shows a sneaky error present in the way we wrote the Iter implementation too!

#### Borrowing and Ownership
Ownership rules mean that, when you move stuff, you can't use it anymore. For some types, this is really good. For example, Box manages an allocation on the heap, and it would be bad if that spot of memory was dropped twice. However, this does not suit other types. i32, for example is just a number that are readily available and don't need that kind of allocation and deallocaction strategy. This is why integers are marked as Copy, a trait which means something can be perfectly recreated from bitwise copy, meaning we can reuse them after they move. This is why you don't need to move Copy out of a reference without replacement!

All numeric primitives in rust are Copy, and any user-defined type can be Copy as long as all its components are Copy. 

Shared references are another example of Copy! Since `&` is a Copy, `Option<&>` is also Copy. When we did `self.next.map` before, it was fine because the Option was just copied. But that strategy won't work here, because `&mut` isn't Copy (copying an &mut would give two different &mut's to the same location, which is unsafe). Instead, we should be `take`ing the Option to get it

```
fn next(&mut self) -> Option<Self::Item> {
        self.next.take.map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }

```
And with that change, the whole thing works. We have IterMut!

# 3. A Persistent Singly-Linked Stack
Now, we'll move from single ownership to shared ownership by writing a persistent immutable singly linked list. This is the same list that is used in functional programming languages. This is equally introduction to `Rc` and `Arc` as it is introduction to the next chapter, which changes things a lot. 

## Layout
The most important thing about a persistent list is that you can manipulate the tails of the lists basically for free.

For example, you might see this kind of thing

```
list1 = A -> B -> C -> D
list2 = tail(list1) = B -> C -> D
list3 = push(list2, x) = X -> B -> C -> D
```
But, at the end we want the memory to look like this from those operations:
```
list1 -> A ---+
              |          
              |
list2 ------->B -> C -> D
              |
              |          
list3 -> X ---+
```

Boxes can't do this, because the ownership of B is shared in this memory model. This kind of problem is what is usually solved with garbage collection. Rust doesn't have anything like that, but it does have reference counting, which can be thought of like a very simply GC. 

The Rc type is just like Box, but it can be duplicated, and the memory gets freed when all the RCs are dropped. This flexibility does come at a cost, however: we can only take a shared reference to its internals, meaning we can never really get data out of one of our lists, nor can we mutate them. 

Changing our implementation starts with copying the singly linked list over and find-replacing Box with Rc
```
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {head: None }
    }
}
```

However, instead of 

## Implement Drop
Previously, we solved the problem of Drop by doing something like this with a mutable list
```
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}
```

However, that cur_link won't work anymore. cur_link = boxed_node.next.take() is mutating the Node inside the Box, but RC doesn't let us do that! We need to rely on shared access, because any number of otehr Rc's might be pointing at the same memory.

But, if we know that we're the last list that knows about this node, it woud actually be fine to move the Node out of the Rc. Then, we could also know when to stop (viz. whenever we can't hoist out of the Node). As it turns out, Rc's `try_unwrap()` method does exactly this
```
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}
```

## Including Arc
One reason that you'd want to use an immutable linked list is to share data across threads. After all, shared mutable state is the root of all evil. The problem is that our current implementation is not thread safe at all. Two threads, at this point, could try to increment the reference count, and only one would happen, which could result in the list being freed sooner than it should be, and then you have a use after freed error. 

In order to get thread safety, we have to use Arc. Arc is completely identical to Rc, but reference counts are modified atomically. The reason why there's even a separate way to do this is simple - atomic reference counting has overhead that Rc does not, so reach for Rc if your program is single-threaded. 

This book makes the claim that "you can't mess up thread-safety in Rust", which, like, idk about that one chief. The claim rests on the idea that rust models thread-safety in a first-class way with the traits Send and Sync.
A type is Send if it's safe to move to another thread. A type is Sync if it is safe to share between multiple threads. That is, if type `T` is Sync, then `&T` is Send. 

Sync and Send are marker traits, so they do not carry any kind of implementations. Send and Sync are automatically derived traits based on whether a product type is totally composed of Send and Sync types. This is similar to how you can only implement Copy if you're made of Copy. 

Almost every type is Send and Sync. Most types are Send because they totally own their data. Most types are Sync because the only way to share data across threads is to put them behind a shared reference, which is what makes them immutable! However, there are special types that violate this property via *interior mutability*. Most of what we've dealt with so far has been *inherited mutability*, aka external mutability. In external mutability, the mutability of a value is inherited from the mutability of its container. That is to say, you can't just mutate some field of a non-mutable value just because you feel like it. 

Interior mutability types violate that idea; they let you mutate through a shared reference. There are two major classes of interior mutability:
1. Cells, which only work in a single-threaded context
2. Locks, which work in a multi-threaded context. 

Cells, naturally, are cheaper to use. 

So, what does this all have to do with Rc and Arc? Both utilize interior mutability for their refernce count! Worse, this reference count is shared between every instance. Rc just uses a cell, which means it's not thread safe. Arc uses an atomic, which means it is thread-safe. You can't magically make a type thread safe by putting it into an Arc. Arc can only derive thread-safety, like any other type. 

# A Bad but Safe Doubly Linked Deque
This chapter mostly exists to show why doing this approach is a bad idea. 

The key to this design is the `RefCell` type, which has a pair of methods that allow for borrowing and mutable borrowing. These function identically to the normal borrowing rules, except these are enforced at runtime rather than comppile time. RefCell will panic and crash a program if you break the rules. 

With the power of Rc and RefCell together, we have the power to make "an incredibly verbose pervasively mutable garbage collected langauge that can't collect cycles"... yay?

Since we're making this doubly linked, each node will have a pointer to the previous and next node. Also, the list itself will have a pointer to the first and last node. This gives us fast insertion and removal on both ends of the list

Here's our first effort at this:
```
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
```

