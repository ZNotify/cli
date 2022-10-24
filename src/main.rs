use clap::{Command, crate_authors, crate_version};
use tokio::runtime::Runtime;

mod send;

fn main() {
    let matches = Command::new("ZNotify")
        .bin_name("znotify")
        .display_name("ZNotify")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Send notification to yourself.")
        .arg_required_else_help(true)
        .subcommand(send::send_command())
        .get_matches();
    match matches.subcommand() {
        Some(("send", sub_matches)) =>
            {
                Runtime::new().unwrap().block_on(send::send_action(sub_matches));
            }
        _ => {}
    }
}
