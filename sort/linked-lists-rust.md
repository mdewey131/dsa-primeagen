# Making Linked Lists in Rust
Because we're looking at the linked lists portion of the DSA Course, I'm also going to bring in the [Rust book on learning linked lists](https://rust-unofficial.github.io/too-many-lists/). I don't think this is a necessary detour, but it will be a way for me to build on all of the stuff I've been learning. As such, this markdown file exists to detail my process of going through that book at a cursory level, learning the trappings of different types of linked lists in Rust.

It is probably worth mentioning that the author of the book linked generally recommends against using linked lists in rust. There are other data structures which emulate the functionality. The rust documentation on the LinkedList data structure also mentions that a Vec or VecDeque is a better solution in Rust because "array-based containers are generally faster, more memory efficient, and make better use of the CPU cache."

## Pros and Cons of Linked Lists
Linked lists, as mentioned in the DSA course, perform their operations to insert, append, and remove in constant time. However, this book argues that such an approach is not really worth it if the time it takes to access some object deep in the list is longer than the time it takes to simply copy and reallocate,. Linked lists, then, are a good option if profiling reveals a lot of time spent splitting and merging list elements

The memory savings that are possible really depends on the object size too. "If you have huge elements, can't predict the load, and have a decent allocator, there are memory savings to be had"

Linked lists are the heroes of the "dark world of lock-free concurrency", but we're unlikely to ever need to build something like that.

# 1. A Bad Singly-Linked Stack
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