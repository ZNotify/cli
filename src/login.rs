use clap::{Arg, ArgMatches, Command};
use znotify::Client;
use crate::config::update_config;

pub(crate) fn login_command() -> Command {
    let user_id_arg = Arg::new("user_id")
        .short('u')
        .long("user_id")
        .required(false)
        .help("User ID to login");
    let endpoint_arg = Arg::new("endpoint")
        .short('e')
        .long("endpoint")
        .required(false)
        .default_value("https://push.learningman.top")
        .help("Endpoint of ZNotify server");
    Command::new("login")
        .short_flag('l')
        .about("Login to ZNotify")
        .args(&[user_id_arg, endpoint_arg])
}

pub(crate) async fn login_action(args: &ArgMatches) {
    let mut user_id = args.get_one::<String>("user_id");
    let mut user_id_in = String::new();
    if user_id.is_none() {
        println!("Please input your user ID:");
        std::io::stdin().read_line(&mut user_id_in).unwrap();
        user_id_in = user_id_in.trim().to_string().to_owned();
        user_id = Some(&user_id_in);
    }
    let user_id = user_id.unwrap().to_owned();

    let endpoint: String = args.get_one::<String>("endpoint").unwrap().to_owned();

    println!("Checking user ID...");

    let ret = Client::create(user_id.clone(), Some(endpoint.clone())).await;
    if ret.is_err() {
        eprintln!("Error: {}", ret.err().unwrap());
        return;
    }

    update_config(user_id.clone(), Some(endpoint.clone()));
    println!("Login success, write config to ~/.znotify/config.toml");
}