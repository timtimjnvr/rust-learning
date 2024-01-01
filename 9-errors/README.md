- By default, if a program pannics, rust cleans up the stack. Rust allows to disable this feature to make the programm immediately abort in case of panic :

```
[profile.release]
panic = 'abort'
```

- to get backtrace of a call that panics, use `RUST_BACKTRACE=1 cargo run`. Debug symbols need to be enabled to do this (enabled by default). They are not enables when running in release mode (`--release` flag)
- the buffer overread is not possible in rust (unlike C).

- the `?` operator : allows to return from a function with the given return type if the `Result` of an expression is an `Err`. It can only be used in functions which return type is compatible with value the `?` is used on (`Result` or `Option`).
- to convert a `Result` into an `Option` and vice versa, you need to use `ok` method on `Result` and reciprocally the method `ok_or` on `Option`.

- `Box<dyn Error>`` means “any kind of error in main function”.
- when the main function returns a `Result<T,E>`, the executable will return 0 if `Ok` and a non zero value if main returns an `Err` (same as in C).
- the main function can return any type that has the adequate function (termination trait) which returns an exit code.

# how

- when you know a function call can't fail it's acceptable to call `expect`.
- it's ok to panic if your code is in a bad state (something unexpected as opposed to something that could likely happen)
