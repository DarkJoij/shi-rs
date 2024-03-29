mod fs;
mod io;
mod macros;
mod utils;
mod commands;

use crate::fs::config::read_config;
use crate::io::args::Args;
use crate::utils::{Re, set_up};

use casual_logger::Log;
use crate::commands::executor::Executor;

fn main() -> Re<()> {
    let args = Args::new()?;
    let mut config = read_config()?;

    set_up(&mut config)?;
    info!("Logger connected!");
    info!("Program stopped with exit code 0.");

    let executor = Executor::new(args); // Try `cargo run -- -s --allow-lalala=true`
    let result = executor.execute(&mut config)?;
    println!("{result}");

    Log::flush();
    Ok(())
}
