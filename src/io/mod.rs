pub mod args;
pub mod logn_macros;

use std::process::Command;

/// Checks if there is access to git through the system shell tools.
pub fn git_enabled() -> bool {
    let git_v = Command::new("git")
        .arg("-v")
        .output();

    matches!(git_v, Ok(..))
}
