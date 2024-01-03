- every reference has a lifetime (after which it becomes invalid and does not reference anything).
- aim of lifetimes is to prevent _dangling references_ (reference other data rather than data the program is intended to reference).
- when returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.
- lifetimes on struct fields force reference it depends on to be valid as long as the struct is valid.
- **Ellision rules** had been added to the compiler to infer some obvious lifetime references.
- lifetimes on function or method parameters are called **input lifetimes parameters**.
- lifetimes on return values are called **output lifetimes parameters**.

# Lifetime Ellision rules

If at the end of the 3 rules, the compiler can't figure out some lifetime, the code won't compile.

1. the compiler applies a different lifetime to each reference parameter of a function.
2. if there is exactly one input lifetime parameter, the compiler applies this lifetime parameter to every output lifetime parameters.
3. if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` (method), then the lifetime of `self` is applied to all output lifetime parameters.

- **static lifetime** are for references that can leave for the entire program duration.
