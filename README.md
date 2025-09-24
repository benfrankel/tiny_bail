# tiny_bail

[![Crates.io](https://img.shields.io/crates/v/tiny_bail.svg?style=for-the-badge)](https://crates.io/crates/tiny_bail)
[![Docs](https://img.shields.io/docsrs/tiny_bail/latest?style=for-the-badge)](https://docs.rs/tiny_bail/latest/tiny_bail/)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?style=for-the-badge)](https://github.com/benfrankel/tiny_bail)

Bailing is an error-handling pattern that takes the middle path between `unwrap` and `?`:
- Compared to `unwrap`: Bailing will `return`, `continue`, or `break` instead of panicking.
- Compared to `?`: Bailing will log or quietly discard the error instead of returning it.

The middle path avoids unwanted panics without the ergonomic challenges of propagating errors with `?`.

This crate provides the following macro variants to determine the preferred behavior on failure:
- [`or_return!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.or_return.html)
- [`or_return_quiet!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.or_return_quiet.html)
- [`or_return_log_once!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.or_return_log_once.html)
- [`or_continue!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.or_continue.html)
- [`or_continue_quiet!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.or_continue_quiet.html)
- [`or_continue_log_once!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.or_continue_log_once.html)
- [`or_break!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.or_break.html)
- [`or_break_quiet!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.or_break_quiet.html)
- [`or_break_log_once!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.or_break_log_once.html)

Along with their tiny aliases:
[`r!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.r.html),
[`rq!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.rq.html),
[`ro!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.ro.html),
[`c!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.c.html),
[`cq!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.cq.html),
[`co!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.co.html),
[`b!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.b.html),
[`bq!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.bq.html), and
[`bo!`](https://docs.rs/tiny_bail/latest/tiny_bail/macro.bo.html).

The macros support `Result`, `Option`, and `bool` types out of the box. You can implement
[`IntoResult`](https://docs.rs/tiny_bail/latest/tiny_bail/trait.IntoResult.html) to extend this to other types.

# Example

```rust
use tiny_bail::prelude::*;

// With `tiny_bail`:
fn increment_last(arr: &mut [i32]) {
    *r!(arr.last_mut()) += 1;
}

// Without `tiny_bail`:
fn increment_last_manually(arr: &mut [i32]) {
    if let Some(x) = arr.last_mut() {
        *x += 1;
    } else {
        tracing::warn!("Bailed at src/example.rs:34:18: `arr.last_mut()` is `None`");
        return;
    }
}
```

# Getting started

To use this crate, add it to your `Cargo.toml`:

```shell
cargo add tiny_bail
```

You can set features to customize the logging behavior on bail:

```shell
# Log with `println!` instead of `tracing::warn!`.
cargo add tiny_bail --no-default-features
# Log with `log::info!` instead of `tracing::warn!`.
cargo add tiny_bail --no-default-features --features log,info
```

This crate has zero dependencies other than the logging backend you choose (`log`, `tracing`, or nothing).

# License

This crate is available under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-Apache-2.0) at your choice.
