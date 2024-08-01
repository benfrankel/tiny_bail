# tiny_bail

[![Crates.io](https://img.shields.io/crates/v/tiny_bail.svg)](https://crates.io/crates/tiny_bail)
[![Docs](https://docs.rs/tiny_bail/badge.svg)](https://docs.rs/tiny_bail/latest/tiny_bail/)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/benfrankel/tiny_bail)

Bailing is an error-handling pattern that takes the middle path between `unwrap` and `?`:
- Compared to `unwrap`: Bail will `return` or `continue` instead of panicking.
- Compared to `?`: Bail will log or ignore the error instead of propagating it.

The middle path avoids unwanted panics without the ergonomic challenges of propagating errors with `?`.

This crate provides four macro variants:
[`r!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.r.html),
[`rq!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.rq.html),
[`c!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.c.html), and
[`cq!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.cq.html); along with their long-form aliases
[`or_return!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.or_return.html),
[`or_return_quiet!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.or_return_quiet.html),
[`or_continue!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.or_continue.html), and
[`or_continue_quiet!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.or_continue_quiet.html), respectively.

```rust
use tiny_bail::prelude::*;

/// Increment the last number of a list, or warn if it's empty.
fn increment_last(list: &mut [i32]) {
    // With `r!`:
    *r!(list.last_mut()) += 1;

    // Without `r!`:
    if let Some(x) = list.last_mut() {
        *x += 1;
    } else {
        println!("Bailed at src/example.rs:34:18: `list.last_mut()`");
        return;
    }
}
```

The macros support `bool`, `Option`, and `Result` types out-of-the-box. This can be extended by implementing
the [`Success`](https://docs.rs/tiny_bail/latest/tiny_bail/trait.Success.html) trait for other types.

You can specify a return value as an optional first argument to the macro, or omit it to default to
`Default::default()`—which even works in functions with no return value.

# License

This crate is available under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-Apache-2.0) at your choice.
