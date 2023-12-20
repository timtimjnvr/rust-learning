- data is stored on the heap (amount of data can grow and shrink as the program runs). There is 3 types of collections:
    - vector : store values **next** to each other.
    - string : ```String``` type.
    - hash map : associate a value with a specific key.

# Vectors
- iterating over a vector is safe because of the borrow checker's rules. Rust prevents simulteanous modifications of the wole vector.
- vectors can store enums to group different types (type need to be known at compile time), you will need to use a match expression to 
handle vector's elements.
