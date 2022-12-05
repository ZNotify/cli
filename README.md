# Notify-cli
Send message to ZNotify server with cli.

## Installation

### Windows

```powershell
winget install Zxilly.NotifyCli
```

### Debian
```bash
# Download from release
sudo apt install znotify-cli_*.deb
```

### Other

Download from release,grant permission and place it in your PATH.

## Usage

```bash
Send notification to ZNotify.

Usage: znotify [COMMAND]

Commands:
  send  Send notification to yourself
  help  Print this message or the help of the given subcommand(s)

Usage: znotify send [OPTIONS] --user_id <user_id> --content <content>

Options:
  -u, --user_id <user_id>  User ID to send notification
  -t, --title <title>      Title of the notification [default: Notification]
  -c, --content <content>  Content of the notification
  -l, --long <long>        Long content of notification
  -h, --help               Print help information
```
