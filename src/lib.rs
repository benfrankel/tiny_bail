//! Tiny bailing convenience macros.
//!
//! Bailing is an error-handling pattern that takes the middle path between `unwrap` and `?`:
//! - Compared to `unwrap`: Bail will `return` or `continue` instead of panicking.
//! - Compared to `?`: Bail will log or ignore the error instead of propagating it.
//!
//! The middle path avoids unwanted panics without the ergonomic challenges of propagating errors with `?`.
//!
//! This crate provides four bailing macro variants:
//! [`r!`],
//! [`rq!`],
//! [`c!`], and
//! [`cq!`]; along with their long-form aliases
//! [`or_return!`],
//! [`or_return_quiet!`],
//! [`or_continue!`], and
//! [`or_continue_quiet!`], respectively.
//!
//! ```rust
//! use tiny_bail::prelude::*;
//!
//! /// Increment the last number of a list, or warn if it's empty.
//! fn increment_last(list: &mut [usize]) {
//!     // With `r!`:
//!     *r!(list.last_mut()) += 1;
//!
//!     // Without `r!`:
//!     if let Some(x) = list.last_mut() {
//!         *x += 1;
//!     } else {
//!         warn!("Bailed at src/example.rs:34:18 `list.last_mut()`");
//!         return;
//!     }
//! }
//! ```
//!
//! The macros support `bool`, `Option`, and `Result` types out-of-the-box. This can be extended by implementing
//! the [`Success`] trait for other types.
//!
//! You can specify the return value as an optional first argument to the macro, or omit it to default to
//! `Default::default()`—which even works in functions with no return value.

/// Re-exported macros.
///
/// The recommended way to use this crate is to glob import the prelude:
///
/// ```rust
/// use tiny_bail::prelude::*;
/// ```
pub mod prelude {
    pub use super::{c, cq, or_continue, or_continue_quiet, or_return, or_return_quiet, r, rq};
}

/// An extension trait for extracting success from failure types.
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

// TODO: Features to choose between `log` and `tracing`, or no logging at all.
// TODO: Features to choose the log level.
// TODO: Log the actual error if it's a `Result::Err`? (what if it doesn't impl `Debug`?)
/// Log relevant information on bail.
#[doc(hidden)]
#[macro_export]
macro_rules! __log_on_bail {
    ($expr:expr) => {
        tracing::warn!(
            "Bailed at {}:{}:{}: `{}`",
            file!(),
            line!(),
            column!(),
            stringify!($expr),
        );
    };
}

// TODO: Explain return value: default, or a provided value.
/// Unwrap or return with a warning.
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

/// A long-form alias for [`r!`].
#[doc(alias = "r")]
#[macro_export]
macro_rules! or_return {
    ($tt:tt) => {
        $crate::r!($tt);
    };
}

// TODO: Explain return value: default, or a provided value.
/// Unwrap or return quietly.
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

/// A long-form alias for [`rq!`].
#[doc(alias = "rq")]
#[macro_export]
macro_rules! or_return_quiet {
    ($tt:tt) => {
        $crate::rq!($tt);
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

/// A long-form alias for [`c!`].
#[doc(alias = "c")]
#[macro_export]
macro_rules! or_continue {
    ($tt:tt) => {
        $crate::c!($tt);
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

/// A long-form alias for [`cq!`].
#[doc(alias = "cq")]
#[macro_export]
macro_rules! or_continue_quiet {
    ($tt:tt) => {
        $crate::cq!($tt);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn r() {
        fn bail_true() -> usize {
            let x = r!(true);
            assert_eq!(x, ());
            5
        }

        fn bail_false() -> usize {
            r!(false);
            5
        }

        fn bail_some() -> usize {
            r!(Some(5))
        }

        fn bail_none() -> usize {
            r!(None)
        }

        fn bail_ok() -> usize {
            r!(Ok::<_, ()>(5))
        }

        fn bail_err() -> usize {
            r!(Err(()))
        }

        assert_eq!(bail_true(), 5);
        assert_eq!(bail_false(), 0);
        assert_eq!(bail_some(), 5);
        assert_eq!(bail_none(), 0);
        assert_eq!(bail_ok(), 5);
        assert_eq!(bail_err(), 0);
    }

    #[test]
    fn rq() {
        fn bail_true() -> usize {
            let x = rq!(true);
            assert_eq!(x, ());
            5
        }

        fn bail_false() -> usize {
            rq!(false);
            5
        }

        fn bail_some() -> usize {
            rq!(Some(5))
        }

        fn bail_none() -> usize {
            rq!(None)
        }

        fn bail_ok() -> usize {
            rq!(Ok::<_, ()>(5))
        }

        fn bail_err() -> usize {
            rq!(Err(()))
        }

        assert_eq!(bail_true(), 5);
        assert_eq!(bail_false(), 0);
        assert_eq!(bail_some(), 5);
        assert_eq!(bail_none(), 0);
        assert_eq!(bail_ok(), 5);
        assert_eq!(bail_err(), 0);
    }

    #[test]
    fn c() {
        fn bail_true() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                let x = c!(true);
                assert_eq!(x, ());
                val = 5;
            }
            val
        }

        fn bail_false() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                c!(false);
                val = 5;
            }
            val
        }

        fn bail_some() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                val = c!(Some(5));
            }
            val
        }

        fn bail_none() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                val = c!(None);
            }
            val
        }

        fn bail_ok() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                val = c!(Ok::<_, ()>(5));
            }
            val
        }

        fn bail_err() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                val = c!(Err(()));
            }
            val
        }

        assert_eq!(bail_true(), 5);
        assert_eq!(bail_false(), 3);
        assert_eq!(bail_some(), 5);
        assert_eq!(bail_none(), 3);
        assert_eq!(bail_ok(), 5);
        assert_eq!(bail_err(), 3);
    }

    #[test]
    fn cq() {
        fn bail_true() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                let x = cq!(true);
                assert_eq!(x, ());
                val = 5;
            }
            val
        }

        fn bail_false() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                cq!(false);
                val = 5;
            }
            val
        }

        fn bail_some() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                val = cq!(Some(5));
            }
            val
        }

        fn bail_none() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                val = cq!(None);
            }
            val
        }

        fn bail_ok() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                val = cq!(Ok::<_, ()>(5));
            }
            val
        }

        fn bail_err() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                val = cq!(Err(()));
            }
            val
        }

        assert_eq!(bail_true(), 5);
        assert_eq!(bail_false(), 3);
        assert_eq!(bail_some(), 5);
        assert_eq!(bail_none(), 3);
        assert_eq!(bail_ok(), 5);
        assert_eq!(bail_err(), 3);
    }
}
