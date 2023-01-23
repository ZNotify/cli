use std::io::stdin;
use clap::Args;
use async_trait::async_trait;
use znotify::Client;
use crate::utils::Action;
use crate::helper::config;
use crate::helper::config::{Config, update_config};

#[derive(Args)]
pub(crate) struct LoginArgs {
    /// User secret to login
    #[arg(short, long)]
    user_secret: Option<String>,
    /// Endpoint of ZNotify server
    #[arg(short, long, default_value = "https://push.learningman.top")]
    endpoint: Option<String>
}

#[async_trait]
impl Action for LoginArgs {
    async fn execute(&self) {
        let mut user_secret = self.user_secret.to_owned();
        let endpoint = self.endpoint.to_owned();
        if user_secret.is_none() {
            println!("Please input your user secret, usually a UUID:");
            let mut user_secret_input = String::new();
            stdin().read_line(&mut user_secret_input).unwrap();
            user_secret_input = user_secret_input.trim().to_string().to_owned();
            user_secret = Some(user_secret_input);
        }
        let user_secret = user_secret.unwrap();

        let config  = config::get_config();
        let endpoint = endpoint.unwrap_or(config.endpoint.unwrap());

        println!("Checking user ID...");

        let ret = Client::create(user_secret.clone(), Some(endpoint.clone())).await;
        if ret.is_err() {
            eprintln!("Error: {}", ret.err().unwrap());
            return;
        }

        update_config(Config{
            user_secret: Some(user_secret.to_owned()),
            endpoint: Some(endpoint.to_owned()),
        });
        println!("Login success, write config to ~/.znotify/config.toml")
    }
}
