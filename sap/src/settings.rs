use std::fmt::{Display, Formatter, Result};

type _ST = String;

#[derive(Debug)]
pub enum Rules {
    Args(_ST),
    Flags(_ST),
    Kwargs(_ST)
}

impl Display for Rules {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            Rules::Args(prefix) => format!("Regular arguments starts with \"{prefix}\"."),
            Rules::Flags(prefix) => format!("Important parameters starts with \"{prefix}\"."),
            Rules::Kwargs(prefix) => format!("Valuable objects (lvalue) starts with \"{prefix}\".")
        })
    }
}
