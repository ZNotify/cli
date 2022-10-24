use clap::{Arg, ArgMatches, Command};
use znotify::send;

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

pub(crate) async fn send_action(args: &ArgMatches) {
    let user_id: String = args.get_one::<String>("user_id").expect("User ID is required").to_owned();
    let title: Option<String> = args.get_one::<String>("title").map(|s| s.to_owned());
    let content = args.get_one::<String>("content").expect("Content is required").to_owned();
    let long = args.get_one::<String>("long").map(|s| s.to_owned());
    let ret = send(user_id, content, title, long).await;
    if ret.is_ok() {
        let msg = ret.unwrap();
        println!("Notification sent");
        println!("ID: {}", msg.id);
        println!("Title: {}", msg.title);
        println!("Content: {}", msg.content);
        println!("Long: {}", msg.long);
        println!("Time: {}", msg.created_at);
    } else {
        eprintln!("Error: {}", ret.err().unwrap());
    }
}