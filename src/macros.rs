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

/// Executes passed blocks only
/// if program runs in `release` mode.
/// The opposite of [`if_debug`] macro.
#[macro_export]
macro_rules! if_release {
    ($release_block:block else $else_block:block) => {{
        if !cfg!(debug_assertions) {
            $release_block
        } else {
            $else_block
        }
    }};
    ($($body:tt)*) => {{
        #[cfg(not(debug_assertions))]
        {
            $($body)*
        }
    }};
}

/// Returns [`Result::Err`] with formatted passed string.
#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        Err(crate::utils::Error::System(format!($($arg)*)))
    };
}

/// Returns [`Result::Ok`] with formatted passed string.
#[macro_export]
macro_rules! ok {
    ($($arg:tt)*) => {
        Ok(crate::utils::Error::System(format!($($arg)*)))
    };
}
