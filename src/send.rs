use clap::{Arg, Command};

pub(crate) fn send_command() -> Command {
    let title_arg = Arg::new("title")
        .required(false)
        .short('t')
        .long("title")
        .default_value("Notification")
        .help("Title of the notification");
    let content_arg = Arg::new("content")
        .short('c')
        .long("content")
        .required(true)
        .help("Content of the notification");
    let long_arg = Arg::new("long")
        .short('l')
        .long("long")
        .required(false)
        .help("Long content of notification");
    let user_id_arg = Arg::new("user_id")
        .short('u')
        .long("user_id")
        .required(true)
        .help("User ID to send notification");
    Command::new("send")
        .about("Send notification to yourself")
        .args(&[user_id_arg, title_arg, content_arg, long_arg])
}