mod fs;
mod io;
mod macros;
mod utils;
mod commands;

use crate::fs::config::read_config;
use crate::io::args::Args;
use crate::utils::{Re, set_up};

use casual_logger::Log;

fn main() -> Re<()> {
    let _args = Args::new()?; // Try `cargo run hehe -test --arg=one`
    let mut config = read_config()?;

    set_up(&mut config)?;

    info!("Logger connected!");
    info!("Program stopped with exit code 0.");

    Log::flush();
    Ok(())
}
