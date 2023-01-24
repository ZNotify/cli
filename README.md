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
Send notification to ZNotify server

Usage: znotify [OPTIONS] <COMMAND>

Commands:
  upgrade  upgrade ZNotify to latest
  login    Log into ZNotify
  send     Send a message to ZNotify
  help     Print this message or the help of the given subcommand(s)

Options:
      --generate <GENERATE>  Generate completion file for shell [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help                 Print help
  -V, --version              Print version
```
