//! Tiny bailing convenience macros.
//!
//! Bailing is an error-handling pattern that takes the middle path between `unwrap` and `?`:
//! - Compared to `unwrap`: Bail will `return`, `continue`, or `break` instead of panicking.
//! - Compared to `?`: Bail will log or ignore the error instead of propagating it.
//!
//! The middle path avoids unwanted panics without the ergonomic challenges of propagating errors with `?`.
//!
//! # Getting started
//!
//! This crate provides six macro variants:
//! - [`or_return!`]
//! - [`or_return_quiet!`]
//! - [`or_continue!`]
//! - [`or_continue_quiet!`]
//! - [`or_break!`]
//! - [`or_break_quiet!`]
//!
//! Along with their tiny aliases:
//! [`r!`](prelude::r),
//! [`rq!`](prelude::rq),
//! [`c!`](prelude::c),
//! [`cq!`](prelude::cq),
//! [`b!`](prelude::b), and
//! [`bq!`](prelude::bq).
//!
//! The macros support [`Result`], [`Option`], and [`bool`] types out of the box.
//! Implement [`IntoResult`] to extend this to other types.
//!
//! # Example
//!
//! ```rust
//! use tiny_bail::prelude::*;
//!
//! // With `tiny_bail`:
//! fn increment_last(arr: &mut [i32]) {
//!     *r!(arr.last_mut()) += 1;
//! }
//!
//! // Without `tiny_bail`:
//! fn increment_last_manually(arr: &mut [i32]) {
//!     if let Some(x) = arr.last_mut() {
//!         *x += 1;
//!     } else {
//!         println!("Bailed at src/example.rs:34:18: `arr.last_mut()`");
//!         return;
//!     }
//! }
//! ```

/// Re-exported macros and tiny aliases.
///
/// To omit tiny aliases, glob import [explicit] instead.
///
/// # Usage
///
/// ```rust
/// use tiny_bail::prelude::*;
/// ```
pub mod prelude {
    pub use super::explicit::*;

    /// Tiny alias for [`or_return!`].
    pub use or_return as r;

    /// Tiny alias for [`or_return_quiet!`].
    pub use or_return_quiet as rq;

    /// Tiny alias for [`or_continue!`].
    pub use or_continue as c;

    /// Tiny alias for [`or_continue_quiet!`].
    pub use or_continue_quiet as cq;

    /// Tiny alias for [`or_break!`].
    pub use or_break as b;

    /// Tiny alias for [`or_break_quiet!`].
    pub use or_break_quiet as bq;
}

/// Re-exported macros.
///
/// To include tiny aliases, glob import [prelude] instead.
///
/// # Usage
///
/// ```
/// use tiny_bail::explicit::*;
/// ```
pub mod explicit {
    pub use super::{
        or_break, or_break_quiet, or_continue, or_continue_quiet, or_return, or_return_quiet,
    };
}

// Verify that the log level feature combination is sane.
#[cfg(any(
    all(feature = "trace", feature = "debug"),
    all(feature = "trace", feature = "info"),
    all(feature = "trace", feature = "warn"),
    all(feature = "trace", feature = "error"),
    all(feature = "debug", feature = "info"),
    all(feature = "debug", feature = "warn"),
    all(feature = "debug", feature = "error"),
    all(feature = "info", feature = "warn"),
    all(feature = "info", feature = "error"),
    all(feature = "warn", feature = "error"),
))]
compile_error!("multiple log level features set (trace, debug, info, warn, error)");

#[cfg(not(any(
    feature = "trace",
    feature = "debug",
    feature = "info",
    feature = "warn",
    feature = "error",
)))]
compile_error!("no log level feature set (trace, debug, info, warn, error)");

// Verify that the log backend feature combination is sane.
#[cfg(all(feature = "log", feature = "tracing"))]
compile_error!("multiple log backend features set (log, tracing)");

/// Set the log backend to `println`.
#[doc(hidden)]
#[cfg(not(any(feature = "log", feature = "tracing")))]
pub mod __log_backend {
    pub use std::{
        println as trace, println as debug, println as info, println as warn, println as error,
    };
}

/// Set the log backend to `log`.
#[doc(hidden)]
#[cfg(feature = "log")]
pub mod __log_backend {
    pub use log::{debug, error, info, trace, warn};
}

/// Set the log backend to `tracing`.
#[doc(hidden)]
#[cfg(feature = "tracing")]
pub mod __log_backend {
    pub use tracing::{debug, error, info, trace, warn};
}

/// Set the log level.
macro_rules! set_log_level {
    ($level:ident) => {
        /// Log the position and error of a bail.
        #[doc(hidden)]
        #[macro_export]
        macro_rules! ___log_on_bail {
            ($expr:expr, $err:expr) => {
                $crate::__log_backend::$level!(
                    "Bailed at {}:{}:{}: `{}` is `{:?}`",
                    file!(),
                    line!(),
                    column!(),
                    stringify!($expr),
                    $err,
                );
            };
        }

        /// Workaround for https://github.com/rust-lang/rust/pull/52234.
        #[doc(hidden)]
        pub use ___log_on_bail as __log_on_bail;
    };
}

#[cfg(feature = "trace")]
set_log_level!(trace);
#[cfg(feature = "debug")]
set_log_level!(debug);
#[cfg(feature = "info")]
set_log_level!(info);
#[cfg(feature = "warn")]
set_log_level!(warn);
#[cfg(feature = "error")]
set_log_level!(error);

/// An extension trait for separating success and failure values.
pub trait IntoResult<T, E> {
    /// Return the success or failure value as a `Result`.
    fn into_result(self) -> Result<T, E>;
}

impl IntoResult<bool, bool> for bool {
    fn into_result(self) -> Result<bool, bool> {
        self.then_some(true).ok_or(false)
    }
}

impl<T> IntoResult<T, Option<()>> for Option<T> {
    fn into_result(self) -> Result<T, Option<()>> {
        self.ok_or(None)
    }
}

impl<T, E> IntoResult<T, E> for Result<T, E> {
    fn into_result(self) -> Result<T, E> {
        self
    }
}

/// Unwrap or return with a warning.
///
/// Returns `Default::default()` unless an initial argument is provided to return instead.
#[macro_export]
macro_rules! or_return {
    ($return:expr, $expr:expr $(,)?) => {
        match $crate::IntoResult::into_result($expr) {
            Ok(x) => x,
            Err(e) => {
                $crate::__log_on_bail!($expr, e);
                return $return;
            }
        }
    };

    ($expr:expr $(,)?) => {
        match $crate::IntoResult::into_result($expr) {
            Ok(x) => x,
            Err(e) => {
                $crate::__log_on_bail!($expr, e);
                return Default::default();
            }
        }
    };
}

/// Unwrap or return quietly.
///
/// Returns `Default::default()` unless an initial argument is provided to return instead.
#[macro_export]
macro_rules! or_return_quiet {
    ($return:expr, $expr:expr $(,)?) => {
        match $crate::IntoResult::into_result($expr) {
            Ok(x) => x,
            _ => return $return,
        }
    };

    ($expr:expr $(,)?) => {
        match $crate::IntoResult::into_result($expr) {
            Ok(x) => x,
            _ => return Default::default(),
        }
    };
}

/// Unwrap or continue with a warning.
///
/// Accepts an optional 'label as the first argument.
#[macro_export]
macro_rules! or_continue {
    ($label:tt, $expr:expr $(,)?) => {
        match $crate::IntoResult::into_result($expr) {
            Ok(x) => x,
            Err(e) => {
                $crate::__log_on_bail!($expr, e);
                continue $label;
            }
        }
    };

    ($expr:expr $(,)?) => {
        match $crate::IntoResult::into_result($expr) {
            Ok(x) => x,
            Err(e) => {
                $crate::__log_on_bail!($expr, e);
                continue;
            }
        }
    };
}

/// Unwrap or continue quietly.
///
/// Accepts an optional 'label as the first argument.
#[macro_export]
macro_rules! or_continue_quiet {
    ($label:tt, $expr:expr $(,)?) => {
        match $crate::IntoResult::into_result($expr) {
            Ok(x) => x,
            _ => continue $label,
        }
    };

    ($expr:expr $(,)?) => {
        match $crate::IntoResult::into_result($expr) {
            Ok(x) => x,
            _ => continue,
        }
    };
}

/// Unwrap or break with a warning.
///
/// Accepts an optional 'label as the first argument.
#[macro_export]
macro_rules! or_break {
    ($label:tt, $expr:expr $(,)?) => {
        match $crate::IntoResult::into_result($expr) {
            Ok(x) => x,
            Err(e) => {
                $crate::__log_on_bail!($expr, e);
                break $label;
            }
        }
    };

    ($expr:expr $(,)?) => {
        match $crate::IntoResult::into_result($expr) {
            Ok(x) => x,
            Err(e) => {
                $crate::__log_on_bail!($expr, e);
                break;
            }
        }
    };
}

/// Unwrap or break quietly.
///
/// Accepts an optional 'label as the first argument.
#[macro_export]
macro_rules! or_break_quiet {
    ($label:tt, $expr:expr $(,)?) => {
        match $crate::IntoResult::into_result($expr) {
            Ok(x) => x,
            _ => break $label,
        }
    };

    ($expr:expr $(,)?) => {
        match $crate::IntoResult::into_result($expr) {
            Ok(x) => x,
            _ => break,
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn r() {
        fn bail_bool(x: bool) -> i32 {
            assert!(or_return!(x));
            5
        }

        fn bail_option(x: Option<i32>) -> i32 {
            assert_eq!(or_return!(x), 1);
            5
        }

        fn bail_result(x: Result<i32, ()>) -> i32 {
            assert_eq!(or_return!(x), 1);
            5
        }

        // Success cases should fall through and output 5.
        assert_eq!(bail_bool(true), 5);
        assert_eq!(bail_option(Some(1)), 5);
        assert_eq!(bail_result(Ok(1)), 5);

        // Failure cases should return early with the default value 0.
        assert_eq!(bail_bool(false), 0);
        assert_eq!(bail_option(None), 0);
        assert_eq!(bail_result(Err(())), 0);
    }

    #[test]
    fn rq() {
        fn bail_bool(x: bool) -> i32 {
            assert!(or_return_quiet!(x));
            5
        }

        fn bail_option(x: Option<i32>) -> i32 {
            assert_eq!(or_return_quiet!(x), 1);
            5
        }

        fn bail_result(x: Result<i32, ()>) -> i32 {
            assert_eq!(or_return_quiet!(x), 1);
            5
        }

        // Success cases should fall through and output 5.
        assert_eq!(bail_bool(true), 5);
        assert_eq!(bail_option(Some(1)), 5);
        assert_eq!(bail_result(Ok(1)), 5);

        // Failure cases should return early with the default value 0.
        assert_eq!(bail_bool(false), 0);
        assert_eq!(bail_option(None), 0);
        assert_eq!(bail_result(Err(())), 0);
    }

    #[test]
    fn c() {
        fn bail_bool(x: bool) -> i32 {
            let mut val = 1;
            for _ in 0..2 {
                val += 1;
                assert!(or_continue!(x));
                val += 1;
            }
            val
        }

        fn bail_option(x: Option<i32>) -> i32 {
            let mut val = 1;
            for _ in 0..2 {
                val += 1;
                assert_eq!(or_continue!(x), 1);
                val += 1;
            }
            val
        }

        fn bail_result(x: Result<i32, ()>) -> i32 {
            let mut val = 1;
            for _ in 0..2 {
                val += 1;
                assert_eq!(or_continue!(x), 1);
                val += 1;
            }
            val
        }

        // Success cases should fall through and output 5.
        assert_eq!(bail_bool(true), 5);
        assert_eq!(bail_option(Some(1)), 5);
        assert_eq!(bail_result(Ok(1)), 5);

        // Failure cases should continue early and output 3.
        assert_eq!(bail_bool(false), 3);
        assert_eq!(bail_option(None), 3);
        assert_eq!(bail_result(Err(())), 3);
    }

    #[test]
    fn cq() {
        fn bail_bool(x: bool) -> i32 {
            let mut val = 1;
            for _ in 0..2 {
                val += 1;
                assert!(or_continue_quiet!(x));
                val += 1;
            }
            val
        }

        fn bail_option(x: Option<i32>) -> i32 {
            let mut val = 1;
            for _ in 0..2 {
                val += 1;
                assert_eq!(or_continue_quiet!(x), 1);
                val += 1;
            }
            val
        }

        fn bail_result(x: Result<i32, ()>) -> i32 {
            let mut val = 1;
            for _ in 0..2 {
                val += 1;
                assert_eq!(or_continue_quiet!(x), 1);
                val += 1;
            }
            val
        }

        // Success cases should fall through and output 5.
        assert_eq!(bail_bool(true), 5);
        assert_eq!(bail_option(Some(1)), 5);
        assert_eq!(bail_result(Ok(1)), 5);

        // Failure cases should continue early and output 3.
        assert_eq!(bail_bool(false), 3);
        assert_eq!(bail_option(None), 3);
        assert_eq!(bail_result(Err(())), 3);
    }

    #[test]
    fn b() {
        fn bail_bool(x: bool) -> i32 {
            let mut val = 1;
            for _ in 0..2 {
                val += 1;
                assert!(or_break!(x));
                val += 1;
            }
            val
        }

        fn bail_option(x: Option<i32>) -> i32 {
            let mut val = 1;
            for _ in 0..2 {
                val += 1;
                assert_eq!(or_break!(x), 1);
                val += 1;
            }
            val
        }

        fn bail_result(x: Result<i32, ()>) -> i32 {
            let mut val = 1;
            for _ in 0..2 {
                val += 1;
                assert_eq!(or_break!(x), 1);
                val += 1;
            }
            val
        }

        // Success cases should fall through and output 5.
        assert_eq!(bail_bool(true), 5);
        assert_eq!(bail_option(Some(1)), 5);
        assert_eq!(bail_result(Ok(1)), 5);

        // Failure cases should break early and output 2.
        assert_eq!(bail_bool(false), 2);
        assert_eq!(bail_option(None), 2);
        assert_eq!(bail_result(Err(())), 2);
    }

    #[test]
    fn bq() {
        fn bail_bool(x: bool) -> i32 {
            let mut val = 1;
            for _ in 0..2 {
                val += 1;
                assert!(or_break_quiet!(x));
                val += 1;
            }
            val
        }

        fn bail_option(x: Option<i32>) -> i32 {
            let mut val = 1;
            for _ in 0..2 {
                val += 1;
                assert_eq!(or_break_quiet!(x), 1);
                val += 1;
            }
            val
        }

        fn bail_result(x: Result<i32, ()>) -> i32 {
            let mut val = 1;
            for _ in 0..2 {
                val += 1;
                assert_eq!(or_break_quiet!(x), 1);
                val += 1;
            }
            val
        }

        // Success cases should fall through and output 5.
        assert_eq!(bail_bool(true), 5);
        assert_eq!(bail_option(Some(1)), 5);
        assert_eq!(bail_result(Ok(1)), 5);

        // Failure cases should break early and output 2.
        assert_eq!(bail_bool(false), 2);
        assert_eq!(bail_option(None), 2);
        assert_eq!(bail_result(Err(())), 2);
    }
}
