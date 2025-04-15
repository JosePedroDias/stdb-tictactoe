# tic-tac-toe

## Enums / Column values

```
game:result
    0 = not started
    1 = ongoing
    2 = p1 wins
    3 = tie
    4 = p2 wins
```

## Tables

```
Game
    id: u32
    p1: Identity
    p2: Identity
    result: u8
    when: Timestamp
    ready1: bool
    ready2: bool

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
```

## Reducers

```
- play(game_id: u32, pos: u8)
- delete_game(game_id: u32)      // not meant to be called from the client
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

spacetime sql tic-tac-toe "SELECT * FROM game WHERE p1='c20084e43db5f504f6a7c7d25fbc70f44722473634416bd58ea40c13969a320d'"
spacetime sql tic-tac-toe "SELECT * FROM game WHERE p2='0000000000000000000000000000000000000000000000000000000000000000'"
```

## TODO

- improve logic to migrate from game_m to game_n after it ends
- improve rust code
