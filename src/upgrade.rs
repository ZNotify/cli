use clap::{Arg, ArgMatches, Command};
use self_update::backends::github::Update;
use self_update::{cargo_crate_version, get_target};

pub(crate) fn upgrade_command() -> Command {
    let force_flag = Arg::new("force")
        .long("force")
        .required(false)
        .num_args(0)
        .help("Force upgrade");
    Command::new("upgrade")
        .about("Upgrade ZNotify to latest")
        .args(&[force_flag])
}

fn check_winget(force: bool) {
    if !cfg!(windows) {
        return;
    }

    println!("Checking winget status...");

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
    } else {
        println!("ZNotify is not installed by winget.");
    }
}

pub(crate) async fn upgrade_action(args: &ArgMatches) {
    println!("Checking for updates...");

    let force = args.get_one::<bool>("force").unwrap().to_owned();

    check_winget(force);

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
    }).await.expect("Error: Upgrade failed");
}