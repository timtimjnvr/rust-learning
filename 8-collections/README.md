- data is stored on the heap (amount of data can grow and shrink as the program runs). There is 3 types of collections:
    - vector : store values **next** to each other.
    - string : ```String``` type.
    - hash map : associate a value with a specific key.

# Vectors
- iterating over a vector is safe because of the borrow checker's rules. Rust prevents simulteanous modifications of the wole vector.
- vectors can store enums to group different types (type need to be known at compile time), you will need to use a match expression to 
handle vector's elements.

# Strings
- for UTF-8 encoded text (UTF-8 encodes Unicode characters into a sequence of bytes.)
- String is wrapper over a ```Vec<u8>```
- Rust strings does not supports indexing. Indeed, an index into the string’s bytes will not always correlate to a valid Unicode scalar value (some Unicode scalar values are stored on multiple bytes).
- 3 ways to look at one String :
    - bytes :
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    - scalar values : 
['न', 'म', 'स', '्', 'त', 'े'] (list of char)
    - grapheme cluster :
["न", "म", "स्", "ते"] (4 letters that makes the world)
- best way to browse a String is to specify whether you want byte or char (scalar values)
- getting graphme clusters is not provided by the standard library.

# Hash Maps
- stores a mapping of key values, determining how it places this values in memory using a has function (the default function is ```SipHash``` and provides security for DoS attacks involving hash maps).
- hash maps data are stored on the heap.
- for values that implements the ```Copy``` traits, values are copied into the hash map. For other values, the values will be moved and owned by the hash map.
- if we insert references in the hash map, the references needs to be valid at least as long as the hash map is valid.