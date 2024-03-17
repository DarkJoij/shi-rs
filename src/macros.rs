/// Executes passed blocks only
/// if program runs in `debug` mode.
#[macro_export]
macro_rules! if_debug {
    ($($body:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            $($body)*
        }
    }};
}

/// Returns [`Result::Err`] with formatted passed string.
#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        Err(format!($($arg)*))
    };
}

/// Returns [`Result::Ok`] with formatted passed string.
#[macro_export]
macro_rules! ok {
    ($($arg:tt)*) => {
        Ok(format!($($arg)*))
    };
}
