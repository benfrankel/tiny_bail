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
//! The macros support [`bool`], [`Option`], and [`Result`] types out of the box.
//! Implement [`Success`] to extend this to other types.
//!
//! You can specify a return value as an optional first argument to the macro, or omit it to default to
//! [`Default::default()`]—which even works in functions with no return value.
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
compile_error!("multiple log level features set");

#[cfg(not(any(
    feature = "trace",
    feature = "debug",
    feature = "info",
    feature = "warn",
    feature = "error",
)))]
compile_error!("no log level feature set");

// Verify that the log backend feature combination is sane.
#[cfg(all(feature = "log", feature = "tracing"))]
compile_error!("multiple log backend features set");

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

// TODO: Log the actual error value for `Result::Err`? (what if it doesn't impl `Debug`?)
/// Set the log level.
macro_rules! set_log_level {
    ($level:ident) => {
        /// Log relevant info on bail.
        #[doc(hidden)]
        #[macro_export]
        macro_rules! ___log_on_bail {
            ($expr:expr) => {
                $crate::__log_backend::$level!(
                    "Bailed at {}:{}:{}: `{}`",
                    file!(),
                    line!(),
                    column!(),
                    stringify!($expr),
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

/// An extension trait for extracting success from fallible types.
pub trait Success<T> {
    /// Return the success value, or `None` on failure.
    fn success(self) -> Option<T>;
}

impl Success<()> for bool {
    fn success(self) -> Option<()> {
        self.then_some(())
    }
}

impl<T> Success<T> for Option<T> {
    fn success(self) -> Option<T> {
        self
    }
}

impl<T, E> Success<T> for Result<T, E> {
    fn success(self) -> Option<T> {
        self.ok()
    }
}

/// Unwrap or return with a warning.
///
/// Returns `Default::default()` unless an initial argument is provided to return instead.
#[macro_export]
macro_rules! or_return {
    ($return:expr, $expr:expr $(,)?) => {
        match $crate::Success::success($expr) {
            Some(x) => x,
            None => {
                $crate::__log_on_bail!($expr);
                return $return;
            }
        }
    };

    ($expr:expr $(,)?) => {
        match $crate::Success::success($expr) {
            Some(x) => x,
            None => {
                $crate::__log_on_bail!($expr);
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
        match $crate::Success::success($expr) {
            Some(x) => x,
            None => return $return,
        }
    };

    ($expr:expr $(,)?) => {
        match $crate::Success::success($expr) {
            Some(x) => x,
            None => return Default::default(),
        }
    };
}

/// Unwrap or continue with a warning.
#[macro_export]
macro_rules! or_continue {
    ($expr:expr) => {
        match $crate::Success::success($expr) {
            Some(x) => x,
            None => {
                $crate::__log_on_bail!($expr);
                continue;
            }
        }
    };
}

/// Unwrap or continue quietly.
#[macro_export]
macro_rules! or_continue_quiet {
    ($expr:expr) => {
        match $crate::Success::success($expr) {
            Some(x) => x,
            None => continue,
        }
    };
}

/// Unwrap or break with a warning.
#[macro_export]
macro_rules! or_break {
    ($expr:expr $(,)?) => {
        match $crate::Success::success($expr) {
            Some(x) => x,
            None => {
                $crate::__log_on_bail!($expr);
                break;
            }
        }
    };
}

/// Unwrap or break quietly.
#[macro_export]
macro_rules! or_break_quiet {
    ($expr:expr $(,)?) => {
        match $crate::Success::success($expr) {
            Some(x) => x,
            None => break,
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn r() {
        fn bail_true() -> usize {
            let _: () = or_return!(true);
            9
        }

        fn bail_false() -> usize {
            let _: () = or_return!(false);
            9
        }

        fn bail_some() -> usize {
            or_return!(Some(9))
        }

        fn bail_none() -> usize {
            let _: () = or_return!(None);
            9
        }

        fn bail_ok() -> usize {
            or_return!(Ok::<_, ()>(9))
        }

        fn bail_err() -> usize {
            let _: () = or_return!(Err(()));
            9
        }

        assert_eq!(bail_true(), 9);
        assert_eq!(bail_false(), 0);
        assert_eq!(bail_some(), 9);
        assert_eq!(bail_none(), 0);
        assert_eq!(bail_ok(), 9);
        assert_eq!(bail_err(), 0);
    }

    #[test]
    fn rq() {
        fn bail_true() -> usize {
            let _: () = or_return_quiet!(true);
            9
        }

        fn bail_false() -> usize {
            let _: () = or_return_quiet!(false);
            9
        }

        fn bail_some() -> usize {
            or_return_quiet!(Some(9))
        }

        fn bail_none() -> usize {
            let _: () = or_return_quiet!(None);
            9
        }

        fn bail_ok() -> usize {
            or_return_quiet!(Ok::<_, ()>(9))
        }

        fn bail_err() -> usize {
            let _: () = or_return_quiet!(Err(()));
            9
        }

        assert_eq!(bail_true(), 9);
        assert_eq!(bail_false(), 0);
        assert_eq!(bail_some(), 9);
        assert_eq!(bail_none(), 0);
        assert_eq!(bail_ok(), 9);
        assert_eq!(bail_err(), 0);
    }

    #[test]
    fn c() {
        fn bail_true() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = or_continue!(true);
                val = i + 6;
            }
            val
        }

        fn bail_false() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = or_continue!(false);
                val = i + 6;
            }
            val
        }

        fn bail_some() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                val = or_continue!(Some(i + 6));
            }
            val
        }

        fn bail_none() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = or_continue!(None);
                val = i + 6;
            }
            val
        }

        fn bail_ok() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                val = or_continue!(Ok::<_, ()>(i + 6));
            }
            val
        }

        fn bail_err() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = or_continue!(Err(()));
                val = i + 6;
            }
            val
        }

        assert_eq!(bail_true(), 8);
        assert_eq!(bail_false(), 5);
        assert_eq!(bail_some(), 8);
        assert_eq!(bail_none(), 5);
        assert_eq!(bail_ok(), 8);
        assert_eq!(bail_err(), 5);
    }

    #[test]
    fn cq() {
        fn bail_true() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = or_continue_quiet!(true);
                val = i + 6;
            }
            val
        }

        fn bail_false() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = or_continue_quiet!(false);
                val = i + 6;
            }
            val
        }

        fn bail_some() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                val = or_continue_quiet!(Some(i + 6));
            }
            val
        }

        fn bail_none() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = or_continue_quiet!(None);
                val = i + 6;
            }
            val
        }

        fn bail_ok() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                val = or_continue_quiet!(Ok::<_, ()>(i + 6));
            }
            val
        }

        fn bail_err() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = or_continue_quiet!(Err(()));
                val = i + 6;
            }
            val
        }

        assert_eq!(bail_true(), 8);
        assert_eq!(bail_false(), 5);
        assert_eq!(bail_some(), 8);
        assert_eq!(bail_none(), 5);
        assert_eq!(bail_ok(), 8);
        assert_eq!(bail_err(), 5);
    }

    #[test]
    fn b() {
        fn bail_true() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = or_break!(true);
                val = i + 6;
            }
            val
        }

        fn bail_false() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = or_break!(false);
                val = i + 6;
            }
            val
        }

        fn bail_some() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                val = or_break!(Some(i + 6));
            }
            val
        }

        fn bail_none() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = or_break!(None);
                val = i + 6;
            }
            val
        }

        fn bail_ok() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                val = or_break!(Ok::<_, ()>(i + 6));
            }
            val
        }

        fn bail_err() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = or_break!(Err(()));
                val = i + 6;
            }
            val
        }

        assert_eq!(bail_true(), 8);
        assert_eq!(bail_false(), 3);
        assert_eq!(bail_some(), 8);
        assert_eq!(bail_none(), 3);
        assert_eq!(bail_ok(), 8);
        assert_eq!(bail_err(), 3);
    }

    #[test]
    fn bq() {
        fn bail_true() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = or_break_quiet!(true);
                val = i + 6;
            }
            val
        }

        fn bail_false() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = or_break_quiet!(false);
                val = i + 6;
            }
            val
        }

        fn bail_some() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                val = or_break_quiet!(Some(i + 6));
            }
            val
        }

        fn bail_none() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = or_break_quiet!(None);
                val = i + 6;
            }
            val
        }

        fn bail_ok() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                val = or_break_quiet!(Ok::<_, ()>(i + 6));
            }
            val
        }

        fn bail_err() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = or_break_quiet!(Err(()));
                val = i + 6;
            }
            val
        }

        assert_eq!(bail_true(), 8);
        assert_eq!(bail_false(), 3);
        assert_eq!(bail_some(), 8);
        assert_eq!(bail_none(), 3);
        assert_eq!(bail_ok(), 8);
        assert_eq!(bail_err(), 3);
    }
}
