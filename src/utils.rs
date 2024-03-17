use std::fmt::{Display, Formatter, Result as FmtResult};

/// Not RegExp! Shortened from [`Result`]
pub type Re<T> = Result<T, String>;

// The following code is not used:

/// Made for quick re-setting inner type in [`Rules`].
type _ST = String;

#[derive(Debug)]
pub enum Rules {
    Args(_ST),
    Flags(_ST),
    Kwargs(_ST)
}

impl Display for Rules {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", match self {
            Rules::Args(prefix) => format!("Regular arguments starts with \"{prefix}\"."),
            Rules::Flags(prefix) => format!("Important parameters starts with \"{prefix}\"."),
            Rules::Kwargs(prefix) => format!("Valuable objects (lvalue) starts with \"{prefix}\".")
        })
    }
}
