- By default, if a program pannics, rust cleans up the stack. Rust allows to disable this feature to make the programm immediately abort in case of panic :

```
[profile.release]
panic = 'abort'
```

- to get backtrace of a call that panics, use `RUST_BACKTRACE=1 cargo run`. Debug symbols need to be enabled to do this (enabled by default). They are not enables when running in release mode (`--release` flag)
- the buffer overread is not possible in rust (unlike C).
