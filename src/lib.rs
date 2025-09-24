//! Tiny bailing convenience macros.
//!
//! Bailing is an error-handling pattern that takes the middle path between `unwrap` and `?`:
//! - Compared to `unwrap`: Bailing will `return`, `continue`, or `break` instead of panicking.
//! - Compared to `?`: Bailing will log or discard the error instead of returning it.
//!
//! The middle path avoids unwanted panics without the ergonomic challenges of propagating errors with `?`.
//!
//! This crate provides the following macro variants to determine the preferred behavior on failure:
//! - [`or_return!`]
//! - [`or_return_quiet!`]
//! - [`or_return_log_once!`]
//! - [`or_continue!`]
//! - [`or_continue_quiet!`]
//! - [`or_continue_log_once!`]
//! - [`or_break!`]
//! - [`or_break_quiet!`]
//! - [`or_break_log_once!`]
//!
//! Along with their tiny aliases:
//! [`r!`](prelude::r),
//! [`rq!`](prelude::rq),
//! [`ro!`](prelude::ro),
//! [`c!`](prelude::c),
//! [`cq!`](prelude::cq),
//! [`co!`](prelude::co),
//! [`b!`](prelude::b),
//! [`bq!`](prelude::bq), and
//! [`bo!`](prelude::bo).
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
//!
//! # Getting started
//!
//! To use this crate, add it to your `Cargo.toml`:
//!
//! ```shell
//! cargo add tiny_bail
//! ```
//!
//! You can set features to customize the logging behavior on bail:
//!
//! ```shell
//! # Log with `println!` instead of `tracing::warn!`.
//! cargo add tiny_bail --no-default-features
//! # Log with `log::info!` instead of `tracing::warn!`.
//! cargo add tiny_bail --no-default-features --features log,info
//! ```
//!
//! This crate has zero dependencies other than the logging backend you choose (`log`, `tracing`, or nothing).

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

// Require a sane feature combination.
#[cfg(all(feature = "log", feature = "tracing"))]
compile_error!("multiple log backend features are set (log, tracing)");
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
compile_error!("multiple log level features are set (trace, debug, info, warn, error)");
#[cfg(all(
    any(feature = "log", feature = "tracing"),
    not(any(
        feature = "trace",
        feature = "debug",
        feature = "info",
        feature = "warn",
        feature = "error",
    )),
))]
compile_error!(
    "a log backend feature is set (log, tracing), but no log level feature is set (trace, debug, info, warn, error)",
);
#[cfg(all(
    not(any(feature = "log", feature = "tracing")),
    any(
        feature = "trace",
        feature = "debug",
        feature = "info",
        feature = "warn",
        feature = "error",
    ),
))]
compile_error!(
    "a log level feature is set (trace, debug, info, warn, error), but no log backend feature is set (log, tracing)",
);

// Set the log backend.
#[doc(hidden)]
pub mod __log_backend {
    #[cfg(feature = "log")]
    pub use log::{debug, error, info, trace, warn};

    #[cfg(feature = "tracing")]
    pub use tracing::{debug, error, info, trace, warn};

    #[cfg(not(any(feature = "log", feature = "tracing")))]
    pub use std::println;
}

/// Set the log level.
macro_rules! set_log_level {
    ($level:ident) => {
        /// Log the code location, expression, and error on bail.
        #[doc(hidden)]
        #[macro_export]
        macro_rules! ___log_bail {
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

        // Workaround for <https://github.com/rust-lang/rust/pull/52234>.
        #[doc(hidden)]
        pub use ___log_bail as __log_bail;
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
#[cfg(not(any(
    feature = "trace",
    feature = "debug",
    feature = "info",
    feature = "warn",
    feature = "error",
)))]
set_log_level!(println);

/// A trait for types that can be separated into success and failure values.
///
/// This trait is implemented for [`Result`], [`Option`], and [`bool`].
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

/// A helper macro to unwrap on success, or log the failure and do something else.
#[doc(hidden)]
#[macro_export]
macro_rules! __unwrap_or {
    ($expr:expr, $else:expr) => {
        match $crate::IntoResult::into_result($expr) {
            ::core::result::Result::Ok(x) => x,
            ::core::result::Result::Err(__err) => {
                $crate::__log_bail!($expr, __err);
                $else;
            }
        }
    };
}

/// Unwrap on success, or log the failure and return.
///
/// Returns [`Default::default()`] unless an initial argument is provided to return instead.
#[macro_export]
macro_rules! or_return {
    ($return:expr, $expr:expr $(,)?) => {
        $crate::__unwrap_or!($expr, return $return)
    };

    ($expr:expr $(,)?) => {
        $crate::__unwrap_or!($expr, return ::core::default::Default::default())
    };
}

/// Unwrap on success, or log the failure and continue.
///
/// Accepts an optional 'label as the first argument.
#[macro_export]
macro_rules! or_continue {
    ($label:tt, $expr:expr $(,)?) => {
        $crate::__unwrap_or!($expr, continue $label)
    };

    ($expr:expr $(,)?) => {
        $crate::__unwrap_or!($expr, continue)
    };
}

/// Unwrap on success, or log the failure and break.
///
/// Accepts an optional 'label as the first argument.
#[macro_export]
macro_rules! or_break {
    ($label:tt, $expr:expr $(,)?) => {
        $crate::__unwrap_or!($expr, break $label)
    };

    ($expr:expr $(,)?) => {
        $crate::__unwrap_or!($expr, break)
    };
}

/// A helper macro to unwrap on success, or quietly discard the failure and do something else.
#[doc(hidden)]
#[macro_export]
macro_rules! __unwrap_or_quiet {
    ($expr:expr, $else:expr) => {
        match $crate::IntoResult::into_result($expr) {
            ::core::result::Result::Ok(x) => x,
            _ => {
                $else;
            }
        }
    };
}

/// Unwrap on success, or quietly discard the failure and return.
///
/// Returns [`Default::default()`] unless an initial argument is provided to return instead.
#[macro_export]
macro_rules! or_return_quiet {
    ($return:expr, $expr:expr $(,)?) => {
        $crate::__unwrap_or_quiet!($expr, return $return)
    };

    ($expr:expr $(,)?) => {
        $crate::__unwrap_or_quiet!($expr, return ::core::default::Default::default())
    };
}

/// Unwrap on success, or quietly discard the failure and continue.
///
/// Accepts an optional 'label as the first argument.
#[macro_export]
macro_rules! or_continue_quiet {
    ($label:tt, $expr:expr $(,)?) => {
        $crate::__unwrap_or_quiet!($expr, continue $label)
    };

    ($expr:expr $(,)?) => {
        $crate::__unwrap_or_quiet!($expr, continue)
    };
}

/// Unwrap on success, or quietly discard the failure and break.
///
/// Accepts an optional 'label as the first argument.
#[macro_export]
macro_rules! or_break_quiet {
    ($label:tt, $expr:expr $(,)?) => {
        $crate::__unwrap_or_quiet!($expr, break $label)
    };

    ($expr:expr $(,)?) => {
        $crate::__unwrap_or_quiet!($expr, break)
    };
}

/// A helper macro to unwrap on success, or log the first failure and do something else.
#[doc(hidden)]
#[macro_export]
macro_rules! __unwrap_or_log_once {
    ($expr:expr, $else:expr) => {
        match $crate::IntoResult::into_result($expr) {
            ::core::result::Result::Ok(x) => x,
            ::core::result::Result::Err(__err) => {
                static __SHOULD_LOG: ::core::sync::atomic::AtomicBool =
                    ::core::sync::atomic::AtomicBool::new(true);
                if __SHOULD_LOG.swap(false, ::core::sync::atomic::Ordering::Relaxed) {
                    $crate::__log_bail!($expr, __err);
                }
                $else;
            }
        }
    };
}

/// Unwrap on success, or log the first failure and return.
///
/// Returns [`Default::default()`] unless an initial argument is provided to return instead.
#[macro_export]
macro_rules! or_return_log_once {
    ($return:expr, $expr:expr $(,)?) => {
        $crate::__unwrap_or_log_once!($expr, return $return)
    };

    ($expr:expr $(,)?) => {
        $crate::__unwrap_or_log_once!($expr, return ::core::default::Default::default())
    };
}

/// Unwrap on success, or log the first failure and continue.
///
/// Accepts an optional 'label as the first argument.
#[macro_export]
macro_rules! or_continue_log_once {
    ($label:tt, $expr:expr $(,)?) => {
        $crate::__unwrap_or_log_once!($expr, continue $label)
    };

    ($expr:expr $(,)?) => {
        $crate::__unwrap_or_log_once!($expr, continue)
    };
}

/// Unwrap on success, or log the first failure and break.
///
/// Accepts an optional 'label as the first argument.
#[macro_export]
macro_rules! or_break_log_once {
    ($label:tt, $expr:expr $(,)?) => {
        $crate::__unwrap_or_log_once!($expr, break $label)
    };

    ($expr:expr $(,)?) => {
        $crate::__unwrap_or_log_once!($expr, break)
    };
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::IntoResult;

    #[test]
    fn r() {
        fn bail<T: Eq + Debug, E: Debug>(outer: impl IntoResult<T, E>, inner: T) -> i32 {
            assert_eq!(or_return!(outer), inner);
            2
        }

        // Success cases should fall through.
        let success = 2;
        assert_eq!(bail(true, true), success);
        assert_eq!(bail(Some(-1), -1), success);
        assert_eq!(bail(Ok::<_, ()>(-1), -1), success);

        // Failure cases should return early with the default value.
        let failure = 0;
        assert_eq!(bail(false, true), failure);
        assert_eq!(bail(None, -1), failure);
        assert_eq!(bail(Err(()), -1), failure);
    }

    #[test]
    fn r_with_value() {
        fn bail<T: Eq + Debug, E: Debug>(outer: impl IntoResult<T, E>, inner: T) -> i32 {
            assert_eq!(or_return!(1, outer), inner);
            2
        }

        // Success cases should fall through.
        let success = 2;
        assert_eq!(bail(true, true), success);
        assert_eq!(bail(Some(-1), -1), success);
        assert_eq!(bail(Ok::<_, ()>(-1), -1), success);

        // Failure cases should return early with the provided value.
        let failure = 1;
        assert_eq!(bail(false, true), failure);
        assert_eq!(bail(None, -1), failure);
        assert_eq!(bail(Err(()), -1), failure);
    }

    #[test]
    fn rq() {
        fn bail<T: Eq + Debug, E: Debug>(outer: impl IntoResult<T, E>, inner: T) -> i32 {
            assert_eq!(or_return_quiet!(outer), inner);
            2
        }

        // Success cases should fall through.
        let success = 2;
        assert_eq!(bail(true, true), success);
        assert_eq!(bail(Some(-1), -1), success);
        assert_eq!(bail(Ok::<_, ()>(-1), -1), success);

        // Failure cases should return early with the default value.
        let failure = 0;
        assert_eq!(bail(false, true), failure);
        assert_eq!(bail(None, -1), failure);
        assert_eq!(bail(Err(()), -1), failure);
    }

    #[test]
    fn rq_with_value() {
        fn bail<T: Eq + Debug, E: Debug>(outer: impl IntoResult<T, E>, inner: T) -> i32 {
            assert_eq!(or_return_quiet!(1, outer), inner);
            2
        }

        // Success cases should fall through.
        let success = 2;
        assert_eq!(bail(true, true), success);
        assert_eq!(bail(Some(-1), -1), success);
        assert_eq!(bail(Ok::<_, ()>(-1), -1), success);

        // Failure cases should return early with the provided value.
        let failure = 1;
        assert_eq!(bail(false, true), failure);
        assert_eq!(bail(None, -1), failure);
        assert_eq!(bail(Err(()), -1), failure);
    }

    #[test]
    fn ro() {
        fn bail<T: Eq + Debug, E: Debug>(outer: impl IntoResult<T, E>, inner: T) -> i32 {
            assert_eq!(or_return_log_once!(outer), inner);
            2
        }

        // Success cases should fall through.
        let success = 2;
        assert_eq!(bail(true, true), success);
        assert_eq!(bail(Some(-1), -1), success);
        assert_eq!(bail(Ok::<_, ()>(-1), -1), success);

        // Failure cases should return early with the default value.
        let failure = 0;
        assert_eq!(bail(false, true), failure);
        assert_eq!(bail(None, -1), failure);
        assert_eq!(bail(Err(()), -1), failure);
    }

    #[test]
    fn ro_with_value() {
        fn bail<T: Eq + Debug, E: Debug>(outer: impl IntoResult<T, E>, inner: T) -> i32 {
            assert_eq!(or_return_log_once!(1, outer), inner);
            2
        }

        // Success cases should fall through.
        let success = 2;
        assert_eq!(bail(true, true), success);
        assert_eq!(bail(Some(-1), -1), success);
        assert_eq!(bail(Ok::<_, ()>(-1), -1), success);

        // Failure cases should return early with the provided value.
        let failure = 1;
        assert_eq!(bail(false, true), failure);
        assert_eq!(bail(None, -1), failure);
        assert_eq!(bail(Err(()), -1), failure);
    }

    #[test]
    fn c() {
        fn bail<T: Eq + Debug, E: Debug>(outer: impl IntoResult<T, E> + Copy, inner: T) -> i32 {
            let mut val = 0;
            '_a: for _ in 0..2 {
                val += 1;
                for _ in 0..2 {
                    val += 1;
                    assert_eq!(or_continue!(outer), inner);
                    val += 1;
                }
                val += 1;
            }
            val
        }

        // Success cases should fall through.
        let success = 12;
        assert_eq!(bail(true, true), success);
        assert_eq!(bail(Some(-1), -1), success);
        assert_eq!(bail(Ok::<_, ()>(-1), -1), success);

        // Failure cases should continue early to the inner loop.
        let failure = 8;
        assert_eq!(bail(false, true), failure);
        assert_eq!(bail(None, -1), failure);
        assert_eq!(bail(Err(()), -1), failure);
    }

    #[test]
    fn c_with_label() {
        fn bail<T: Eq + Debug, E: Debug>(outer: impl IntoResult<T, E> + Copy, inner: T) -> i32 {
            let mut val = 0;
            '_a: for _ in 0..2 {
                val += 1;
                for _ in 0..2 {
                    val += 1;
                    assert_eq!(or_continue!('_a, outer), inner);
                    val += 1;
                }
                val += 1;
            }
            val
        }

        // Success cases should fall through.
        let success = 12;
        assert_eq!(bail(true, true), success);
        assert_eq!(bail(Some(-1), -1), success);
        assert_eq!(bail(Ok::<_, ()>(-1), -1), success);

        // Failure cases should continue early to the outer loop.
        let failure = 4;
        assert_eq!(bail(false, true), failure);
        assert_eq!(bail(None, -1), failure);
        assert_eq!(bail(Err(()), -1), failure);
    }

    #[test]
    fn cq() {
        fn bail<T: Eq + Debug, E: Debug>(outer: impl IntoResult<T, E> + Copy, inner: T) -> i32 {
            let mut val = 0;
            '_a: for _ in 0..2 {
                val += 1;
                for _ in 0..2 {
                    val += 1;
                    assert_eq!(or_continue_quiet!(outer), inner);
                    val += 1;
                }
                val += 1;
            }
            val
        }

        // Success cases should fall through.
        let success = 12;
        assert_eq!(bail(true, true), success);
        assert_eq!(bail(Some(-1), -1), success);
        assert_eq!(bail(Ok::<_, ()>(-1), -1), success);

        // Failure cases should continue early to the inner loop.
        let failure = 8;
        assert_eq!(bail(false, true), failure);
        assert_eq!(bail(None, -1), failure);
        assert_eq!(bail(Err(()), -1), failure);
    }

    #[test]
    fn cq_with_label() {
        fn bail<T: Eq + Debug, E: Debug>(outer: impl IntoResult<T, E> + Copy, inner: T) -> i32 {
            let mut val = 0;
            '_a: for _ in 0..2 {
                val += 1;
                for _ in 0..2 {
                    val += 1;
                    assert_eq!(or_continue_quiet!('_a, outer), inner);
                    val += 1;
                }
                val += 1;
            }
            val
        }

        // Success cases should fall through.
        let success = 12;
        assert_eq!(bail(true, true), success);
        assert_eq!(bail(Some(-1), -1), success);
        assert_eq!(bail(Ok::<_, ()>(-1), -1), success);

        // Failure cases should continue early to the outer loop.
        let failure = 4;
        assert_eq!(bail(false, true), failure);
        assert_eq!(bail(None, -1), failure);
        assert_eq!(bail(Err(()), -1), failure);
    }

    #[test]
    fn co() {
        fn bail<T: Eq + Debug, E: Debug>(outer: impl IntoResult<T, E> + Copy, inner: T) -> i32 {
            let mut val = 0;
            '_a: for _ in 0..2 {
                val += 1;
                for _ in 0..2 {
                    val += 1;
                    assert_eq!(or_continue_log_once!(outer), inner);
                    val += 1;
                }
                val += 1;
            }
            val
        }

        // Success cases should fall through.
        let success = 12;
        assert_eq!(bail(true, true), success);
        assert_eq!(bail(Some(-1), -1), success);
        assert_eq!(bail(Ok::<_, ()>(-1), -1), success);

        // Failure cases should continue early to the inner loop.
        let failure = 8;
        assert_eq!(bail(false, true), failure);
        assert_eq!(bail(None, -1), failure);
        assert_eq!(bail(Err(()), -1), failure);
    }

    #[test]
    fn co_with_label() {
        fn bail<T: Eq + Debug, E: Debug>(outer: impl IntoResult<T, E> + Copy, inner: T) -> i32 {
            let mut val = 0;
            '_a: for _ in 0..2 {
                val += 1;
                for _ in 0..2 {
                    val += 1;
                    assert_eq!(or_continue_log_once!('_a, outer), inner);
                    val += 1;
                }
                val += 1;
            }
            val
        }

        // Success cases should fall through.
        let success = 12;
        assert_eq!(bail(true, true), success);
        assert_eq!(bail(Some(-1), -1), success);
        assert_eq!(bail(Ok::<_, ()>(-1), -1), success);

        // Failure cases should continue early to the outer loop.
        let failure = 4;
        assert_eq!(bail(false, true), failure);
        assert_eq!(bail(None, -1), failure);
        assert_eq!(bail(Err(()), -1), failure);
    }
    #[test]
    fn b() {
        fn bail<T: Eq + Debug, E: Debug>(outer: impl IntoResult<T, E> + Copy, inner: T) -> i32 {
            let mut val = 0;
            '_a: for _ in 0..2 {
                val += 1;
                for _ in 0..2 {
                    val += 1;
                    assert_eq!(or_break!(outer), inner);
                    val += 1;
                }
                val += 1;
            }
            val
        }

        // Success cases should fall through.
        let success = 12;
        assert_eq!(bail(true, true), success);
        assert_eq!(bail(Some(-1), -1), success);
        assert_eq!(bail(Ok::<_, ()>(-1), -1), success);

        // Failure cases should break early from the inner loop.
        let failure = 6;
        assert_eq!(bail(false, true), failure);
        assert_eq!(bail(None, -1), failure);
        assert_eq!(bail(Err(()), -1), failure);
    }

    #[test]
    fn b_with_label() {
        fn bail<T: Eq + Debug, E: Debug>(outer: impl IntoResult<T, E> + Copy, inner: T) -> i32 {
            let mut val = 0;
            '_a: for _ in 0..2 {
                val += 1;
                for _ in 0..2 {
                    val += 1;
                    assert_eq!(or_break!('_a, outer), inner);
                    val += 1;
                }
                val += 1;
            }
            val
        }

        // Success cases should fall through.
        let success = 12;
        assert_eq!(bail(true, true), success);
        assert_eq!(bail(Some(-1), -1), success);
        assert_eq!(bail(Ok::<_, ()>(-1), -1), success);

        // Failure cases should break early from the outer loop.
        let failure = 2;
        assert_eq!(bail(false, true), failure);
        assert_eq!(bail(None, -1), failure);
        assert_eq!(bail(Err(()), -1), failure);
    }

    #[test]
    fn bq() {
        fn bail<T: Eq + Debug, E: Debug>(outer: impl IntoResult<T, E> + Copy, inner: T) -> i32 {
            let mut val = 0;
            '_a: for _ in 0..2 {
                val += 1;
                for _ in 0..2 {
                    val += 1;
                    assert_eq!(or_break_quiet!(outer), inner);
                    val += 1;
                }
                val += 1;
            }
            val
        }

        // Success cases should fall through.
        let success = 12;
        assert_eq!(bail(true, true), success);
        assert_eq!(bail(Some(-1), -1), success);
        assert_eq!(bail(Ok::<_, ()>(-1), -1), success);

        // Failure cases should break early from the inner loop.
        let failure = 6;
        assert_eq!(bail(false, true), failure);
        assert_eq!(bail(None, -1), failure);
        assert_eq!(bail(Err(()), -1), failure);
    }

    #[test]
    fn bq_with_label() {
        fn bail<T: Eq + Debug, E: Debug>(outer: impl IntoResult<T, E> + Copy, inner: T) -> i32 {
            let mut val = 0;
            '_a: for _ in 0..2 {
                val += 1;
                for _ in 0..2 {
                    val += 1;
                    assert_eq!(or_break_quiet!('_a, outer), inner);
                    val += 1;
                }
                val += 1;
            }
            val
        }

        // Success cases should fall through.
        let success = 12;
        assert_eq!(bail(true, true), success);
        assert_eq!(bail(Some(-1), -1), success);
        assert_eq!(bail(Ok::<_, ()>(-1), -1), success);

        // Failure cases should break early from the outer loop.
        let failure = 2;
        assert_eq!(bail(false, true), failure);
        assert_eq!(bail(None, -1), failure);
        assert_eq!(bail(Err(()), -1), failure);
    }

    #[test]
    fn bo() {
        fn bail<T: Eq + Debug, E: Debug>(outer: impl IntoResult<T, E> + Copy, inner: T) -> i32 {
            let mut val = 0;
            '_a: for _ in 0..2 {
                val += 1;
                for _ in 0..2 {
                    val += 1;
                    assert_eq!(or_break_log_once!(outer), inner);
                    val += 1;
                }
                val += 1;
            }
            val
        }

        // Success cases should fall through.
        let success = 12;
        assert_eq!(bail(true, true), success);
        assert_eq!(bail(Some(-1), -1), success);
        assert_eq!(bail(Ok::<_, ()>(-1), -1), success);

        // Failure cases should break early from the inner loop.
        let failure = 6;
        assert_eq!(bail(false, true), failure);
        assert_eq!(bail(None, -1), failure);
        assert_eq!(bail(Err(()), -1), failure);
    }

    #[test]
    fn bo_with_label() {
        fn bail<T: Eq + Debug, E: Debug>(outer: impl IntoResult<T, E> + Copy, inner: T) -> i32 {
            let mut val = 0;
            '_a: for _ in 0..2 {
                val += 1;
                for _ in 0..2 {
                    val += 1;
                    assert_eq!(or_break_log_once!('_a, outer), inner);
                    val += 1;
                }
                val += 1;
            }
            val
        }

        // Success cases should fall through.
        let success = 12;
        assert_eq!(bail(true, true), success);
        assert_eq!(bail(Some(-1), -1), success);
        assert_eq!(bail(Ok::<_, ()>(-1), -1), success);

        // Failure cases should break early from the outer loop.
        let failure = 2;
        assert_eq!(bail(false, true), failure);
        assert_eq!(bail(None, -1), failure);
        assert_eq!(bail(Err(()), -1), failure);
    }
}
