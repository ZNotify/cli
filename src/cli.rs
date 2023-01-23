use clap::{CommandFactory, Parser};

use crate::command::Commands;
use crate::utils::{Action, print_completions};
use async_trait::async_trait;
use clap_complete::{Shell};
use crate::build;

#[derive(Parser)]
#[command(name = "ZNotify", author, version = build::CLAP_LONG_VERSION, about)]
pub(crate) struct Cli {
    /// Generate completion file for shell
    #[arg(long, value_enum)]
    generate: Option<Shell>,

    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[async_trait]
impl Action for Cli {
    async fn execute(&self) {
        if let Some(generator) = self.generate {
            let mut cmd = Cli::command();
            eprintln!("Generating completion file for {:?}...", generator);
            print_completions(generator, &mut cmd);
            return;
        }

        match &self.command {
            Commands::Upgrade(args) => {
                args.execute().await;
            }
            Commands::Login(args) => {
                args.execute().await;
            }
            Commands::Send(args) => {
                args.execute().await;
            }
        }
    }
}
