mod args;
mod macros;
mod utils;

use crate::args::Args;
use crate::utils::Re;

fn main() -> Re<()> {
    let args = Args::new()?; // Try `cargo run hehe -test --arg=one`
    if_debug!(println!("{args}"));

    Ok(())
}
