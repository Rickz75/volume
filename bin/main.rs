use clap::{Parser, Subcommand};
use volume::Result;

/// A simple CLI to control the master volume on linux systems.
#[derive(Parser)]
#[clap(about = "A CLI used to control the master volume on linux systems", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Sets the master volume to the given amount. Range from 1-100.
    #[clap(arg_required_else_help = true)]
    Set { amount: i64 },
    /// Retrieves the current master volume.
    Get,
    /// Mutes the master volume.
    Mute,
    /// Unmutes the master volume.
    Unmute,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Set { amount } => volume::set(amount)?,
        Command::Get => println!("{}", volume::get()?),
        Command::Mute => volume::mute()?,
        Command::Unmute => volume::unmute()?,
    }

    Ok(())
}
