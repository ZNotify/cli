use clap::{Command, crate_authors, crate_version};
use tokio::runtime::Handle;

mod send;

fn main() {
    let matches = Command::new("ZNotify")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Send notification to yourself.")
        .subcommand(send::send_command())
        .get_matches();
    match matches.subcommand() {
        Some(("send", sub_matches)) =>
            {
                Handle::current().block_on(send::send_action(sub_matches));
            }
        _ => {}
    }
}
