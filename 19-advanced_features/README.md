# Unsafe Rust

- if you use a reference in unsafe code, it will still be checked (borrow checker or other Rust safety checks). The unsafe keyword only gives you access to five features that are then not checked by the compiler for memory safety :
- Raw pointers
- Calling an Unsafe Function or Method
- Creating a Safe Abstraction over Unsafe Code
- Using extern Functions to Call External Code
- Accessing or Modifying a Mutable Static Variable (global variables).
- Implementing an Unsafe Trait
- Accessing Fields of a Union

# Adavanced Traits

- the **difference** between **associated types** and **generics** on traits is that with generics you can implement a trait definiton on a type for almost any type whereas with an associated type we just can't implement a trait on a type multiple times.
- we define a **default generic type parameter** (this eliminates the need for the implementor of the trait to specify a concrete type if the default works).
- when choosing betwenn methods/traits of the same name, the compiler will always choose as a priority the methods directly implemented on the type. In this situation, to specificly call some traits methods we need to use **fully qualified syntax** : `Trait::function_name`.
- **associated functions** are functions on a type that are not methods (don’t have a `self` parameter). When choosing between associated functions and traits implementations of the same name, Rust can't figure out which implementation to choose because associated functions does not have a `self` parameter, and there could be other types that implement the same method. To use a trait rather than the associated function we need to use **fully qualified syntax** : `<ConcreteType as TraitType>::function_name())`.
- general definition of the fully qualified syntax : `<Type as Trait>::function(receiver_if_method, next_arg, ...);`.
- a `supertrait` is a trait some other traits depends on to implement it own logic.
- `newtype pattern` allows to implement a trait on an external type (neither the trait nor the type are local to our crate) : it is a thin wrapper around the type we want to implement a trait for

# Advanced types

- the `!` type is the never type this type never holds a value.
- in Rust it’s not possible to create a variable holding a dynamically sized type (we mananges to handle dynamically sized date behind a pointer of some kind : reference, smart pointers ...)
- Every trait is a dynamically sized type we can refer to by using the name of the trait.

# Advanced Functions and Closures

- closure are dynamically so they can't be returned directly from functions

# Macros

- macros are a way of writing code that writes other code, which is known as **metaprogramming**.
- macros can take a variable number.
- macros can implement a trait on a given type.
- macros need to be defined or brought into scope **before you** call them in a file.
- type of macros :
  - declarative macros
  - procedural macros
