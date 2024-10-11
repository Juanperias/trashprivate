mod api;
mod cli;
mod commands;
mod data;
mod models;
mod train;

use anyhow::Result;
use cli::get_cli;

fn main() -> Result<()> {
    let cli = get_cli();

    cli.commands.exec()?;

    Ok(())
}
