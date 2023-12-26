# Heap & Stack

- stack & heap are both part of the memory.
- stack : stores and removes values *in LIFO mode*. Adding data is called *pushing onto the stack* removing is called *popping off the stack*. data must have *fixed size*.
- all data with unknown size at compile time or size which changes must be stored on the heap.
- to put something on the heap you request for some space and you get a pointer. the *memory allocator* returns you a pointer.
which is the adress to the location.
- pushing to the stack is faster than allocating on the heap.
- accessing data on the heap is slower than accessing data on the heap.
- when you call a function, the values passed to that function and the function local variables get pushed onto the stack. When the function is over those values get popped of the stack.
- main objective of ownership : **to manage heap data**.

# Referenes and borowing

- reference is like a pointer : an adress we can follow to get data from memory, unlike a pointer a reference garantee to point to a valid value of a particular type for the life of that reference.
- reference allows to refer to some value without taking ownership of it.
- values passed by reference are not borrowed so they are not droped when the reference stops being used.
- values passed to function by reference can't be modifiable, you need to use mutable references.
- a mutable variable can only have one mutable reference : this prevents data race at compile time.
- slices is a kind of reference so it does not have ownership.

# Summary
- ownership, borrowing, and slices ensure memory safety in Rust programs at compile time.
- the data is cleaned up from memory automatically when the owner goes out of scope.