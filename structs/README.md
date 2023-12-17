- struct are similar to tuples except they have named fields.
# Ownership
- Structs with ```String``` whithin their fields own this data, this can be use if we want that data to be valid for as long as the entire struct is valid.
- if we use ```str``` instead of ```String```, we need to use the rust *lifetimes* feature.