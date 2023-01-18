use clap::{Arg, ArgMatches, Command};
use znotify::{Client, MessageOptions, entity::Priority};

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
    let user_secret_arg = Arg::new("user_secret")
        .short('u')
        .long("user_secret")
        .required(false)
        .help("User ID to send notification");
    let priority_arg = Arg::new("priority")
        .short('p')
        .long("priority")
        .required(false)
        .default_value("normal")
        .value_parser(["low", "normal", "high"])
        .help("Priority of the notification");
    Command::new("send")
        .short_flag('s')
        .about("Send notification to ZNotify")
        .args(&[user_secret_arg, title_arg, content_arg, long_arg, priority_arg])
}

pub(crate) async fn send_action(args: &ArgMatches) {
    let config = crate::config::get_config();
    let config_user_id = config.user_secret;
    let config_endpoint = config.endpoint;

    let user_id = args.get_one::<String>("user_id");
    if user_id.is_none() && config_user_id.is_none() {
        eprintln!("Error: User ID is not set, thus is required");
        return;
    }
    let user_id = user_id.unwrap_or(&config_user_id.unwrap()).to_owned();

    let title: Option<String> = args.get_one::<String>("title").map(|s| s.to_owned());
    let content = args.get_one::<String>("content").expect("Content is required").to_owned();
    let long = args.get_one::<String>("long").map(|s| s.to_owned());
    let priority = args
        .get_one::<String>("priority")
        .map(|s| s.to_owned())
        .map(|s| match s.as_str() {
            "high" => Priority::High,
            "normal" => Priority::Normal,
            "low" => Priority::Low,
            _ => Priority::Normal,
        });


    let client = Client::create(user_id.clone(), config_endpoint.clone()).await.unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let ret = client.send(MessageOptions {
        title,
        content,
        long,
        priority,
    }).await;
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