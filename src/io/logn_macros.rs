use crate::if_release;
use crate::fs::config::Config;

use casual_logger::Log;

// Levels: `trace` -> `info` -> `warn` -> `fatal`:

/// Logs data at the `trace` level.
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        casual_logger::Log::trace(&format!($($arg)*))
    }
}

/// Logs data at the `info` level.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        casual_logger::Log::info(&format!($($arg)*))
    }
}

/// Logs data at the `warn` level.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        casual_logger::Log::warn(&format!($($arg)*))
    }
}

/// Logs the latest data at the `fatal` level
/// and completes execution with a panic.
#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {{
        casual_logger::Log::fatal(&format!($($arg)*));
        panic!("unhandled: {}", message);
    }}
}

/// Sets systemically important logger parameters.
pub fn set_up_logger(config: &Config) {
    Log::set_retention_days(30);
    Log::set_file_name("shi-rs_journal");
    Log::remove_old_logs();

    if config.logging_allowed {
        if_release! {
            use casual_logger::{Level, Opt};

            Log::set_level(Level::Info);
            Log::set_opt(Opt::Release);
        }
    }
}
