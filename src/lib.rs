//! Tiny failure skipping macros.
// TODO: Expand module-level docs.

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
        match $crate::util::macros::Success::success($expr) {
            Some(x) => x,
            None => {
                $crate::log_on_bail!($expr);
                return $return;
            }
        }
    };

    ($expr:expr $(,)?) => {
        match $crate::util::macros::Success::success($expr) {
            Some(x) => x,
            None => {
                $crate::log_on_bail!($expr);
                return Default::default();
            }
        }
    };
}

// TODO: Explain return value: default, or a provided value.
/// Unwrap or return quietly.
#[macro_export]
macro_rules! rq {
    ($return:expr, $expr:expr $(,)?) => {
        match $crate::util::macros::Success::success($expr) {
            Some(x) => x,
            None => return $return,
        }
    };

    ($expr:expr $(,)?) => {
        match $crate::util::macros::Success::success($expr) {
            Some(x) => x,
            None => return Default::default(),
        }
    };
}

/// Unwrap or continue with a warning.
#[macro_export]
macro_rules! c {
    ($expr:expr) => {
        match $crate::util::macros::Success::success($expr) {
            Some(x) => x,
            None => {
                $crate::log_on_bail!($expr);
                continue;
            }
        }
    };
}

/// Unwrap or continue quietly.
#[macro_export]
macro_rules! cq {
    ($expr:expr) => {
        match $crate::util::macros::Success::success($expr) {
            Some(x) => x,
            None => continue,
        }
    };
}
