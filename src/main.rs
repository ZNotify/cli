use clap::{Command, crate_authors, crate_version};
use utils::run_blocking;

mod send;
mod login;
mod config;
mod utils;
mod upgrade;

fn main() {
    let matches = Command::new("ZNotify")
        .bin_name("znotify")
        .display_name("ZNotify")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Send notification to ZNotify.")
        .arg_required_else_help(true)
        .subcommand(send::send_command())
        .subcommand(login::login_command())
        .subcommand(upgrade::upgrade_command())
        .get_matches();
    match matches.subcommand() {
        Some(("send", sub_matches)) =>
            {
                run_blocking(send::send_action(sub_matches));
            }
        Some(("login", sub_matches)) =>
            {
                run_blocking(login::login_action(sub_matches));
            }
        Some(("upgrade", sub_matches)) =>
            {
                run_blocking(upgrade::upgrade_action(sub_matches));
            }
        _ => {}
    }
}
