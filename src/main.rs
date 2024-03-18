mod fs;
mod io;
mod macros;
mod utils;

use crate::fs::config::read_config;
use crate::io::args::Args;
use crate::io::logn_macros::set_up_logger;
use crate::utils::Re;

use casual_logger::Log;

fn main() -> Re<()> {
    let _args = Args::new()?; // Try `cargo run hehe -test --arg=one`
    let config = read_config()?;

    if config.logging {
        set_up_logger();
        info!("Logger connected!");
    }

    Log::flush();
    Ok(())
}
