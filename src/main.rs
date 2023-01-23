use clap::Parser;
use shadow_rs::shadow;
use crate::utils::{Action, run_blocking};

mod cli;
mod command;
mod utils;
mod helper;

shadow!(build);

fn main() {
    let cli = cli::Cli::parse();

    run_blocking(cli.execute());
}

