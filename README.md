# push-locker
A utility for merge window reservation.

# Motivation
In programming teams, there are often cases when it is necessary to reserve a merge window for the correct merge of a large patchset.
For these purposes, a handy git pre-push hook is needed to notify that the merge window is currently busy.

# Installation

```
git clone https://github.com/Mephistophiles/push-locker
cd push-locker
cargo build --release # cargo install is not working for workspaces
mkdir -p $HOME/.config/pushlock/
echo "server = <server_ip:server_port>" >  $HOME/.config/pushlock/config.toml
```

# Structure
* `pushlock-server` - HTTP server for centralized storage of the state of the merge window.
* `pushlockctl` - command-line tool for the manage merge window
* `pushlock-check` - lightweight tool to check for merge availability.

# Workflow

* Run `pushlock-server` on a dedicated server available to every team member.
* Configure `pushlockctl` for each team member.
* Copy `pre-push` hook from the repo and replace server address in the file.
* Run `pushlockctl lock` to reserve the merge window.
* Run `pushlockctl unlock` to release the currently locked merge window.
* Run `pushlockctl check` to check if the merge is available.
