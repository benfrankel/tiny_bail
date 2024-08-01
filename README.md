# Tiny failure-skipping macros

[![Crates.io](https://img.shields.io/crates/v/tiny_bail.svg)](https://crates.io/crates/tiny_bail)
[![Docs](https://docs.rs/tiny_bail/badge.svg)](https://docs.rs/tiny_bail/latest/tiny_bail/)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/benfrankel/tiny_bail)

This crate provides four failure-skipping macros: `r!`, `rq!`, `c!`, and `cq!`; along with their long-form aliases
`or_return!`, `or_return_quiet!`, `or_continue!`, and `or_continue_quiet!`, respectively.

The macros support both `Option` and `Result` types out-of-the-box. This can be extended by implementing the
`Success` trait for other types.

You can provide a return value as an optional first argument to the macro, or you can omit it to default to
`Default::default()`—which also works in functions with no return value.

# Example

```rust
/// Increment the last number of a list, or warn if it's empty.
fn increment_last(list: &mut [usize]) {
    // With `r!`:
    *r!(list.last_mut()) += 1;

    // Without `r!`:
    if let Some(x) = list.last_mut() {
        *x += 1;
    } else {
        warn!("Empty list");
        return;
    }
}
```

# License

This crate is available under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-Apache-2.0) at your choice.
