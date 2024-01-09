# Closures

- closure decalaration does not need type annotations as Rust infer the types.
- Closures can capture values from their environment by :
  - borrowing immutably (can be accessed multiple time).
  - borrowing mutably (captures a mutable reference, no other borrows are allowed between closure definition and usage. )
  - taking ownership (by using `move`)
- a closure can can :
  - **move** a captured value out of the closure
  - **mutate** the captured value
  - **neither move nor mutate** the value, or capture nothing from the environment to begin with.
- the way a closure captures and handles values affects which traits the closure implements :
  - `FnOnce`: the closure can only be called once. For closures that moves the capture values will (only implements `FnOnce` and can only be called once).
  - `FnMut`: the closure can be called more than once. For closures that don't move captured values out of their body, but that might mutate the captured values (implements `FnMut`)`.
  - `Fn`: can be called multiple times without mutating their environment. For closures that don’t move captured values out of their body and that don’t mutate captured values,

# Iterators

- all iterators implements the `Iterator` trait.
- types of iterators :
  - `iter` : creates an immutable iterator (values retrieved are immutable reference to the values in the vector).
  - `into_iter`: creates a vector that takes ownership of the vector and returns owned values.
  - `iter_mut`: creates a mutable iterator (values retrieved ar mutable reference to the values in the vector).
- some methods consumes iterators, other produce new ones out of an existing one.
