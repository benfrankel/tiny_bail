//! Tiny failure-skipping macros.
// TODO: Expand module-level docs.

/// TODO
pub mod prelude {
    pub use super::{c, cq, or_continue, or_continue_quiet, or_return, or_return_quiet, r, rq};
}

// TODO: Impl `Success<()> for bool`
/// An extension trait for extracting success from failure types.
pub trait Success<T> {
    /// Return the success value, or `None` on failure.
    fn success(self) -> Option<T>;
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
        fn unwrap_some() -> usize {
            r!(Some(5))
        }

        fn unwrap_none() -> usize {
            r!(None)
        }

        fn unwrap_ok() -> usize {
            r!(Ok::<_, ()>(5))
        }

        fn unwrap_err() -> usize {
            r!(Err(()))
        }

        assert_eq!(unwrap_some(), 5);
        assert_eq!(unwrap_none(), 0);
        assert_eq!(unwrap_ok(), 5);
        assert_eq!(unwrap_err(), 0);
    }

    #[test]
    fn rq() {
        fn unwrap_some() -> usize {
            rq!(Some(5))
        }

        fn unwrap_none() -> usize {
            rq!(None)
        }

        fn unwrap_ok() -> usize {
            rq!(Ok::<_, ()>(5))
        }

        fn unwrap_err() -> usize {
            rq!(Err(()))
        }

        assert_eq!(unwrap_some(), 5);
        assert_eq!(unwrap_none(), 0);
        assert_eq!(unwrap_ok(), 5);
        assert_eq!(unwrap_err(), 0);
    }

    #[test]
    fn c() {
        fn unwrap_some() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                val = c!(Some(5));
            }
            val
        }

        fn unwrap_none() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                val = c!(None);
            }
            val
        }

        fn unwrap_ok() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                val = c!(Ok::<_, ()>(5));
            }
            val
        }

        fn unwrap_err() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                val = c!(Err(()));
            }
            val
        }

        assert_eq!(unwrap_some(), 5);
        assert_eq!(unwrap_none(), 3);
        assert_eq!(unwrap_ok(), 5);
        assert_eq!(unwrap_err(), 3);
    }

    #[test]
    fn cq() {
        fn unwrap_some() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                val = cq!(Some(5));
            }
            val
        }

        fn unwrap_none() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                val = cq!(None);
            }
            val
        }

        fn unwrap_ok() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                val = cq!(Ok::<_, ()>(5));
            }
            val
        }

        fn unwrap_err() -> usize {
            let mut val = 3;
            for _ in 0..1 {
                val = cq!(Err(()));
            }
            val
        }

        assert_eq!(unwrap_some(), 5);
        assert_eq!(unwrap_none(), 3);
        assert_eq!(unwrap_ok(), 5);
        assert_eq!(unwrap_err(), 3);
    }
}
