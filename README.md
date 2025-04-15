# tic-tac-toe

The simplest tic tac toe game, in [spacetimedb](https://spacetimedb.com/).

Server code in Rust.  
Brower client code in TypeScript and vanilla DOM interface.


## Tables

```
Game
    id: u32
    p1: Identity
    p2: Identity
    when: Timestamp

GameMove
    id: u32
    game_id: u32
    player_id: Identity
    when: Timestamp
    position: u8

Feedback
    id: u32
    game_id: u32
    player_id: Identity
    when: Timestamp
    message: String

DeleteGameTimer  (private, to trigger delete_game reducer later)
    scheduled_id: u64
    scheduled_at: ScheduleAt
    game_id: u32

PlayerStats
    id: Identity
    starts: u32
    wins: u32
    ties: u32
    losses: u32
    created_at: Timestamp
    updated_at: Timestamp
```

## Reducers

```
- play(game_id: u32, pos: u8)
- delete_game(game_id: u32)      // not meant to be called from the client
- new_game()                     // problematic for now, not in use
```

## Commands

```
spacetime generate --lang typescript --out-dir client-ts/src/module_bindings --project-path server-rs
clear && spacetime publish --project-path server-rs tic-tac-toe --delete-data -y

cd client-ts
npm run build
npm run preview

spacetime logs tic-tac-toe
spacetime sql tic-tac-toe "SELECT * FROM game"
spacetime sql tic-tac-toe "SELECT * FROM game_move"
spacetime sql tic-tac-toe "SELECT * FROM feedback"
spacetime sql tic-tac-toe "SELECT * FROM player_stats"

spacetime sql tic-tac-toe "SELECT * FROM game WHERE p1='c20084e43db5f504f6a7c7d25fbc70f44722473634416bd58ea40c13969a320d'"
spacetime sql tic-tac-toe "SELECT * FROM game WHERE p2='0000000000000000000000000000000000000000000000000000000000000000'"
```

## TODO

- improve rust code (strings, repeated code, make board into a struct?)
- show new game button or fix the new_game reducer usage
