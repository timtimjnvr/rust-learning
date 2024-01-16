- `let` statements, and `for` loops can only accept **irrefutable patterns**.
- `if let` and `while let` expressions accept **refutable and irrefutable patterns**.
- a match expression stops checking arms once it has found the first matching pattern.
- **match guards** are additional conditionnal expressions that need to match for that arm to be choosen.
- compiler doesn't try to check for exhaustiveness when match guard expressions are involved.

https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#-bindings
