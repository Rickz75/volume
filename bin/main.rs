use clap::{Parser, Subcommand};
use volume::Error;

/// Exits the application, displaying the given error to the user.
fn exit(err: Error) -> ! {
    eprintln!("{err}");
    std::process::exit(1)
}

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

fn main() {
    let args = Cli::parse();

    let res = match args.command {
        Command::Set { amount } => volume::set(amount),
        Command::Mute => volume::mute(),
        Command::Unmute => volume::unmute(),
        Command::Get => {
            return match volume::get() {
                Ok(vol) => println!("{}", vol),
                Err(e) => exit(e),
            }
        }
    };

    if let Err(e) = res {
        exit(e);
    }
}
