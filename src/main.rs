mod cli;
mod commands;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    cli::run()
}
