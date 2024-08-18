//! Tiny bailing convenience macros.
//!
//! Bailing is an error-handling pattern that takes the middle path between `unwrap` and `?`:
//! - Compared to `unwrap`: Bail will `return` or `continue` instead of panicking.
//! - Compared to `?`: Bail will log or ignore the error instead of propagating it.
//!
//! The middle path avoids unwanted panics without the ergonomic challenges of propagating errors with `?`.
//!
//! This crate provides six macro variants:
//! [`r!`],
//! [`rq!`],
//! [`c!`],
//! [`cq!`],
//! [`b!`], and
//! [`bq!`]; along with their long-form aliases
//! [`or_return!`],
//! [`or_return_quiet!`],
//! [`or_continue!`],
//! [`or_continue_quiet!`],
//! [`or_break!`], and
//! [`or_break_quiet!`], respectively.
//!
//! ```rust
//! use tiny_bail::prelude::*;
//!
//! /// Increment the last number of a list, or warn if it's empty.
//! fn increment_last(list: &mut [i32]) {
//!     // With `r!`:
//!     *r!(list.last_mut()) += 1;
//!
//!     // Without `r!`:
//!     if let Some(x) = list.last_mut() {
//!         *x += 1;
//!     } else {
//!         println!("Bailed at src/example.rs:34:18: `list.last_mut()`");
//!         return;
//!     }
//! }
//! ```
//!
//! The macros support `bool`, `Option`, and `Result` types out-of-the-box. This can be extended by implementing
//! the [`Success`] trait for other types.
//!
//! You can specify a return value as an optional first argument to the macro, or omit it to default to
//! `Default::default()`—which even works in functions with no return value.

/// Re-exported macros.
///
/// The recommended way to use this crate is to glob import the prelude:
///
/// ```rust
/// use tiny_bail::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        b, bq, c, cq, or_break, or_break_quiet, or_continue, or_continue_quiet, or_return,
        or_return_quiet, r, rq,
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
macro_rules! r {
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

/// The long-form alias of [`r!`].
#[doc(alias = "r")]
#[macro_export]
macro_rules! or_return {
    ($($tt:tt)*) => {
        $crate::r!($($tt)*);
    };
}

/// Unwrap or return quietly.
///
/// Returns `Default::default()` unless an initial argument is provided to return instead.
#[macro_export]
macro_rules! rq {
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

/// The long-form alias of [`rq!`].
#[doc(alias = "rq")]
#[macro_export]
macro_rules! or_return_quiet {
    ($($tt:tt)*) => {
        $crate::rq!($($tt)*);
    };
}

/// Unwrap or continue with a warning.
#[macro_export]
macro_rules! c {
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

/// The long-form alias of [`c!`].
#[doc(alias = "c")]
#[macro_export]
macro_rules! or_continue {
    ($($tt:tt)*) => {
        $crate::c!($($tt)*);
    };
}

/// Unwrap or continue quietly.
#[macro_export]
macro_rules! cq {
    ($expr:expr) => {
        match $crate::Success::success($expr) {
            Some(x) => x,
            None => continue,
        }
    };
}

/// The long-form alias of [`cq!`].
#[doc(alias = "cq")]
#[macro_export]
macro_rules! or_continue_quiet {
    ($($tt:tt)*) => {
        $crate::cq!($($tt)*);
    };
}

/// Unwrap or break with a warning.
#[macro_export]
macro_rules! b {
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

/// The long-form alias of [`b!`].
#[doc(alias = "b")]
#[macro_export]
macro_rules! or_break {
    ($($tt:tt)*) => {
        $crate::b!($($tt)*);
    };
}

/// Unwrap or break quietly.
#[macro_export]
macro_rules! bq {
    ($expr:expr $(,)?) => {
        match $crate::Success::success($expr) {
            Some(x) => x,
            None => break,
        }
    };
}

/// The long-form alias of [`bq!`].
#[doc(alias = "bq")]
#[macro_export]
macro_rules! or_break_quiet {
    ($($tt:tt)*) => {
        $crate::bq!($($tt)*);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn r() {
        fn bail_true() -> usize {
            let _: () = r!(true);
            9
        }

        fn bail_false() -> usize {
            let _: () = r!(false);
            9
        }

        fn bail_some() -> usize {
            r!(Some(9))
        }

        fn bail_none() -> usize {
            let _: () = r!(None);
            9
        }

        fn bail_ok() -> usize {
            r!(Ok::<_, ()>(9))
        }

        fn bail_err() -> usize {
            let _: () = r!(Err(()));
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
            let _: () = rq!(true);
            9
        }

        fn bail_false() -> usize {
            let _: () = rq!(false);
            9
        }

        fn bail_some() -> usize {
            rq!(Some(9))
        }

        fn bail_none() -> usize {
            let _: () = rq!(None);
            9
        }

        fn bail_ok() -> usize {
            rq!(Ok::<_, ()>(9))
        }

        fn bail_err() -> usize {
            let _: () = rq!(Err(()));
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
                let _: () = c!(true);
                val = i + 6;
            }
            val
        }

        fn bail_false() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = c!(false);
                val = i + 6;
            }
            val
        }

        fn bail_some() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                val = c!(Some(i + 6));
            }
            val
        }

        fn bail_none() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = c!(None);
                val = i + 6;
            }
            val
        }

        fn bail_ok() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                val = c!(Ok::<_, ()>(i + 6));
            }
            val
        }

        fn bail_err() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = c!(Err(()));
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
                let _: () = cq!(true);
                val = i + 6;
            }
            val
        }

        fn bail_false() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = cq!(false);
                val = i + 6;
            }
            val
        }

        fn bail_some() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                val = cq!(Some(i + 6));
            }
            val
        }

        fn bail_none() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = cq!(None);
                val = i + 6;
            }
            val
        }

        fn bail_ok() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                val = cq!(Ok::<_, ()>(i + 6));
            }
            val
        }

        fn bail_err() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = cq!(Err(()));
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
                let _: () = b!(true);
                val = i + 6;
            }
            val
        }

        fn bail_false() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = b!(false);
                val = i + 6;
            }
            val
        }

        fn bail_some() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                val = b!(Some(i + 6));
            }
            val
        }

        fn bail_none() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = b!(None);
                val = i + 6;
            }
            val
        }

        fn bail_ok() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                val = b!(Ok::<_, ()>(i + 6));
            }
            val
        }

        fn bail_err() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = b!(Err(()));
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
                let _: () = bq!(true);
                val = i + 6;
            }
            val
        }

        fn bail_false() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = bq!(false);
                val = i + 6;
            }
            val
        }

        fn bail_some() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                val = bq!(Some(i + 6));
            }
            val
        }

        fn bail_none() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = bq!(None);
                val = i + 6;
            }
            val
        }

        fn bail_ok() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                val = bq!(Ok::<_, ()>(i + 6));
            }
            val
        }

        fn bail_err() -> usize {
            let mut val = 9;
            for i in 0..3 {
                val = i + 3;
                let _: () = bq!(Err(()));
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
