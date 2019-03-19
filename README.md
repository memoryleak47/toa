ToA
===

Turn-based Strategy Multiplayer Game written in Rust using SFML.

## How to run
#### In one Process:

```
cd toaserver

cargo run 
```

#### In two other Processes:

```
cd toaclient

cargo run localhost:4242
```

#### Now enter the server-commands

`team <player_id> <team_string>` to change the team of player `<player_id>` to `<team_string>`.

`status` to get status information about the lobby state.

`go` to start the game.

#### Just use `team 0 1` and then `go` to play a normal match.
