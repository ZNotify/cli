mod login;
mod send;
mod upgrade;

use clap::Subcommand;
use crate::command::login::LoginArgs;
use crate::command::upgrade::UpgradeArgs;
use crate::command::send::SendArgs;

#[derive(Subcommand)]
pub(crate) enum Commands {
    #[command(name = "upgrade", about = "upgrade ZNotify to latest")]
    Upgrade(UpgradeArgs),
    #[command(name = "login", about = "Log into ZNotify")]
    Login(LoginArgs),
    #[command(name = "send", about = "Send a message to ZNotify")]
    Send(SendArgs)
}
