use clap::Args;
use async_trait::async_trait;
use self_update::backends::github::Update;
use self_update::{cargo_crate_version, get_target};
use crate::utils::Action;

#[derive(Args)]
pub(crate) struct UpgradeArgs {
    /// Force upgrade, even if ZNotify is installed by package manager
    #[arg(short, long)]
    force: bool,
}

#[async_trait]
impl Action for UpgradeArgs {
    async fn execute(&self) {
        println!("Checking for updates...");

        let force = self.force;

        check_winget(force);
        check_apt(force);

        let bin_name = format!("znotify-{}", get_target());

        tokio::task::spawn_blocking(move || {
            let status = Update::configure()
                .repo_owner("ZNotify")
                .repo_name("cli")
                .bin_name(bin_name.as_str())
                .current_version(cargo_crate_version!())
                .build().unwrap()
                .update()
                .unwrap();
            if status.updated() {
                println!("Update success, updated to {}", status.version());
            } else if status.uptodate() {
                println!("Already up to date");
            } else {
                println!("No update available");
            }
        }).await.expect("Upgrade failed");
    }
}

fn check_apt(force: bool) {
    if !cfg!(unix) {
        return;
    }

    // check is debian or ubuntu
    let output = std::process::Command::new("lsb_release")
        .args(&["-a"])
        .output();
    if output.is_err() {
        return;
    }
    let output = output.unwrap();
    let output = String::from_utf8_lossy(output.stdout.as_ref());
    let ret = output.contains("Ubuntu") || output.contains("Debian");
    if !ret {
        return;
    }

    let output = std::process::Command::new("apt")
        .args(&["list", "--installed", "znotify-cli"])
        .output();
    if output.is_err() {
        return;
    }

    let output = output.unwrap();
    let output = String::from_utf8_lossy(output.stdout.as_ref());
    let ret = output.contains("znotify-cli");
    if ret && !force {
        println!("ZNotify is installed by apt, please use `apt upgrade znotify-cli` or download deb file to upgrade.");
        println!("If you still want to use `znotify upgrade` to upgrade, please use force flag: `znotify upgrade --force`");
        std::process::exit(0);
    } else if ret {
        println!("ZNotify is installed by apt, but you still want to use `znotify upgrade` to upgrade.");
    }

}

fn check_winget(force: bool) {
    if !cfg!(windows) {
        return;
    }

    let output = std::process::Command::new("winget")
        .args(&["list", "--id", "Zxilly.NotifyCli"])
        .output();
    if output.is_err() {
        return;
    }
    let output = output.unwrap();
    let output = String::from_utf8_lossy(output.stdout.as_ref());
    let ret = output.contains("Zxilly.NotifyCli");
    if ret && !force {
        println!("ZNotify is installed by winget, please use `winget upgrade Zxilly.NotifyCli` to upgrade.");
        println!("If you still want to use `znotify upgrade` to upgrade, please use force flag: `znotify upgrade --force`");
        std::process::exit(0);
    } else if ret {
        println!("ZNotify is installed by winget, but you still want to use `znotify upgrade` to upgrade.");
    }
}
