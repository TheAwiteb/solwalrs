//! # Why macros?
//! Because we want to log messages only if the `--verbose` flag is set,
//! and we don't want to call functions we don't need to call,
//! so we use macros to call the functions only if the `--verbose` flag is set.

/// A trait to check if a value is true, used in macros
pub trait IsTrue {
    fn is_true(&self) -> bool;
}

impl<T, E> IsTrue for Result<T, E> {
    fn is_true(&self) -> bool {
        self.is_ok()
    }
}

impl<T> IsTrue for Option<T> {
    fn is_true(&self) -> bool {
        self.is_some()
    }
}

/// Loging info macro, if the `--verbose` flag is set
#[macro_export]
macro_rules! info {
    ($args:expr, $($arg:expr),*) => {
        let args: &$crate::app::AppArgs = $args;
        if args.verbose {
            // put the arguments into a format macro
            prettylog::info(&format!("{};{}: {}", file!(), line!(), format!($($arg),*)));
        }
    }
}

/// Loging error macro, if the `--verbose` flag is set
#[macro_export]
macro_rules! error {
    ($args:expr, $($arg:expr),*) => {
        let args: &$crate::app::AppArgs = $args;
        if args.verbose {
            // put the arguments into a format macro
            prettylog::error(&format!("{};{}: {}", file!(), line!(), format!($($arg),*)));
        }
    }
}

/// Loging warn macro, if the `--verbose` flag is set
#[macro_export]
macro_rules! warn {
    ($args:expr, $($arg:expr),*) => {
        let args: &$crate::app::AppArgs = $args;
        if args.verbose {
            // put the arguments into a format macro
            prettylog::warn(&format!("{};{}: {}", file!(), line!(), format!($($arg),*)));
        }
    }
}

/// login `info` if the Result/Option is `Ok`/`Some`, or `warn` if the `Result`/`Option` is `Err`/`None`
/// ### Example
/// ```rust
/// use solwalrs::info_or_warn;
/// info_or_warn!(args, Ok(1), "The result is 1"; "The result is not 1");
/// info_or_warn!(args, Some(1), "The result is 1"; "The result is not 1");
/// info_or_warn!(args, Err(1), "The result is 1"; "The result is not 1");
/// info_or_warn!(args, None, "The result is 1"; "The result is not 1");
/// ```
/// #### Output
/// ```text
/// [INFO] The result is 1
/// [INFO] The result is 1
/// [WARN] The result is not 1
/// [WARN] The result is not 1
/// ```
#[macro_export]
macro_rules! info_or_warn {
    ($args:expr, $result:expr, $($okargs:expr),*; $($errargs:expr),*) => {
        use $crate::log::IsTrue;
        if $result.is_true() {
            $crate::info!($args, $($okargs),*);
        } else {
            $crate::warn!($args, $($errargs),*);
        }
    }
}
