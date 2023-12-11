- stack & heap are both part of the memory.
- stack : stores and removes values *in LIFO mode*. Adding data is called *pushing onto the stack* removing is called *popping off 
the stack*. data must have *fixed size*.
- all data with unknown size at compile time or size which changes must be stored on the heap.
- to put something on the heap you request for some space and you get a pointer. the *memory allocator* returns you a pointer 
which is the adress to the location.
- pushing to the stack is faster than allocating on the heap.
- accessing data on the heap is slower than accessing data on the heap.
- when you call a function, the values passed to that function and the function local variables get pushed on to the stack. When 
the function is over those values get popped of the stack.
- main objective of ownership : **to manage heap data**.
