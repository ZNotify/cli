use std::fmt::{Display, Formatter};
use clap::{Args, ValueEnum};
use async_trait::async_trait;
use znotify::{Client, MessageOptions, entity::Priority as ZPriority};
use crate::helper::config::get_config;
use crate::utils::Action;

#[derive(ValueEnum, Clone)]
enum Priority {
    /// Low priority, will not show on screen, may not be received on low battery devices
    Low,
    /// Normal priority, will show on screen
    Normal,
    /// High priority, will show on screen and make sound
    High,
}

impl Display for Priority {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Normal => write!(f, "normal"),
            Priority::High => write!(f, "high"),
        }
    }
}

#[derive(Args)]
pub(crate) struct SendArgs {
    /// Title of the notification
    #[arg(short, long, default_value = "Notification")]
    title: Option<String>,
    /// Content of the notification
    /// This is required
    #[arg(short, long)]
    content: String,
    /// Long content of notification
    /// Content can be in markdown format
    #[arg(short, long)]
    long: Option<String>,
    /// User Secret to send notification
    /// If not set, will use the user secret in config
    #[arg(short, long)]
    user_secret: Option<String>,
    /// Priority of the notification
    /// Can be low, normal or high
    #[arg(short, long, default_value_t = Priority::Normal)]
    priority: Priority,
    /// Endpoint of the ZNotify server
    /// If not set, will use the endpoint in config
    /// If not set in config, will use the default endpoint (https://push.learningman.top)
    #[arg(short, long)]
    endpoint: Option<String>,
}

#[async_trait]
impl Action for SendArgs{
    async fn execute(&self) {
        let config = get_config();

        if self.user_secret.is_none() && config.user_secret.is_none() {
            eprintln!("Error: User secret is not set in config, thus is required in command line");
            std::process::exit(1);
        }
        let user_secret;
        if self.user_secret.is_some(){
            user_secret = self.user_secret.to_owned();
        } else {
            user_secret = config.user_secret.to_owned();
        }

        let endpoint = self.endpoint.clone().unwrap_or(config.endpoint.unwrap());

        let client = Client::create(user_secret.unwrap(), Some(endpoint)).await.unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        });

        let ret = client.send(
            MessageOptions{
                content: self.content.to_owned(),
                title: self.title.to_owned(),
                long: self.long.to_owned(),
                priority: priority_map(self.priority.to_owned()),
            }
        ).await;

        if ret.is_ok() {
            let msg = ret.unwrap();
            println!("Notification sent");
            println!("ID: {}", msg.id);
            println!("Title: {}", msg.title);
            println!("Content: {}", msg.content);
            println!("Long: {}", msg.long);
            println!("Priority: {}", msg.priority.to_string());
        } else {
            eprintln!("Error: {}", ret.err().unwrap());
        }
    }
}

fn priority_map(p: Priority) -> Option<ZPriority> {
    match p {
        Priority::Low => Some(ZPriority::Low),
        Priority::Normal => Some(ZPriority::Normal),
        Priority::High => Some(ZPriority::High),
    }
}
