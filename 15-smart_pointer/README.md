# Intro

- pointers borrow the value they points to. Rust pointers are called reference.
- smart pointers own the data they point to. They act like a pointer but also have **additional metadata and capabilities**.
- `String` and `Vec<T>` are smart pointers, they both own some data.
- smart pointers implement the `Deref` and `Drop` traits :
  - `Deref`: allow an instance of smart pointer to act like a reference.
  - `Drop`: allow to define custom behaviour when an instance of smart pointer goes out of scope.
- most common smart pointers :
  - `Box<T>` for allocating values on the heap
  - `Rc<T>`, a reference counting type that enables multiple ownership
  - `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time

# Box<T>

- boxes allow to store data on the heap rather than the stack. The pointer to the heap data remains on the stack.
- implementing recursive types with values is impossible in Rust because the value size need to be known at compile time. However by using a `Box<T>` to reference recursive type solves the problem because Rust knows the size of a Box<T>

# Treating Smart pointers like regular references

- to be able to dereference our own smart pointer with `*` we need to implement the `Deref` trait : one method named `deref` that borrows self and returns a reference to the inner data.
- the `deref` method return a reference to the value, otherwise the value would be moved out of `self`, (Most of the time we don't want to take ownership of the value when we dereference)

```
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

# Implicit Deref Coercions with Functions and Methods

- **Deref coercion** converts a reference to a type that implements the `Deref` trait into a reference to another type (ex: `&String` into a `&str`).
- Deref coercion allows Rust to handle type conversions for us automatically using `deref` as many time as necessary.
- `DerefMut` trait override the `*` on mutables reference.

# Freeing memory

- the method `drop` that takes a mutable reference to `self` will be called every time a smart pointer goes out of scope.
- the `drop` method can't be called automatically to free an object. For that you have to call the `std::mem::drop` to force a value to be dropped before the end of its scope (it's included in the prelude). The Rust compiler will ensure the value is not still being used.

# Rc<T>, the Reference Counted Smart Pointer

- in general, you know exactly which variable owns a given value.
- `Rc<T>` can be used to allocate data on the heap for multiple part of our program to read and we can't determine at compile time which part will finish using this data (and could be data owner).
- `Rc<T>` allows a single value to have multiple owners (via immutable references).
- `Rc::clone`, is used to increments the reference count. The data won't be cleaned up until the reference count is zero.

# RefCell<T> and the Interior Mutability Pattern

- **Interior mutability** allows you to mutate data even when there are immutable references to that data (normally, it is disallowed by the borrowing rules).
- `RefCell<T>` type represents single ownership over the data it holds.
- Unlike `Box<T>` With` RefCell<T>`, these borrowing rules are enforced at runtime (if you break these rules, your program will panic and exit.):
  - at any given time, you can have either (but not both) one mutable reference or any number of immutable references.
  - references must always be valid.
- If you have an `Rc<T>` that holds a `RefCell<T>`, you can get a value that can have multiple owners and that you can mutate!

# Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>

- Strong references are how you can share ownership of an `Rc<T>` instance.
- `Rc::clone` increases the `strong_count` of an `Rc<T>` instance, and an `Rc<T>` instance is only cleaned up if its `strong_count` is 0.
- Weak references don't express an ownership and does not affect `strong_count` so using them can prevent the reference cycle.
- To create a weak reference out of a reference contained by `Rc<T>`, we use the `Rc::downgrade` function to create a `Weak<T>`.

# Summary

- the `Box<T>` type has a **known size** and points to data allocated on the heap.
- the `Rc<T>` type keeps track of the **number of references to data on the heap** so that data can have **multiple owners**.
- the `RefCell<T>` type with its **interior mutability** gives us a type that we can use when we need an immutable type but need **to change an inner value** of that type; it also **enforces the borrowing rules at runtime** instead of at compile time.
- the `Weak<T>` is a pointer that does not affect the number of references of Rc<T> prevents references cycles (memory leaks).
