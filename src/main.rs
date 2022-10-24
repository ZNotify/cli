use clap::{Arg, Command, crate_authors, crate_version};
use znotify::send as notify_send;

mod send;

fn main() {
    let matches = Command::new("ZNotify")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Send notification to yourself.")
        .subcommand(send::send_command())
        .get_matches();
    match matches.subcommand() {
        ("send", Some(sub_matches)) => {
            let user_id = sub_matches.value_of("user_id").unwrap();
            let title = sub_matches.value_of("title").unwrap();
            let content = sub_matches.value_of("content").unwrap();
            let long = sub_matches.value_of("long").unwrap();
            notify_send(user_id, title, content, long);
        }
        _ => {}
    }
}
