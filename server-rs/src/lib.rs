use spacetimedb::{Identity, ReducerContext, ScheduleAt, Table, TimeDuration, Timestamp};
use std::time::Duration;

/////// TABLES

#[spacetimedb::table(name = game, public)]
#[derive(Debug, Clone)]
pub struct Game {
    #[auto_inc]
    #[primary_key]
    id: u32,
    #[index(btree)]
    p1: Identity,
    #[index(btree)]
    p2: Identity,
    when: Timestamp,
}

#[spacetimedb::table(name = player_stats, public)]
#[derive(Debug, Clone)]
pub struct PlayerStats {
    #[primary_key]
    id: Identity,
    starts: u32,
    left_early: u32,
    wins: u32,
    ties: u32,
    losses: u32,
    created_at: Timestamp,
    updated_at: Timestamp,
}

#[spacetimedb::table(name = game_move, public)]
#[derive(Debug, Clone)]
pub struct GameMove {
    #[auto_inc]
    #[primary_key]
    id: u32,
    #[index(btree)]
    game_id: u32,
    player_id: Identity,
    when: Timestamp,
    position: u8,
}

#[spacetimedb::table(name = feedback, public)]
#[derive(Debug, Clone)]
pub struct Feedback {
    #[auto_inc]
    #[primary_key]
    id: u32,
    #[index(btree)]
    game_id: u32,
    #[index(btree)]
    player_id: Identity,
    when: Timestamp,
    message: String,
}

#[spacetimedb::table(name = delete_game_timer, scheduled(delete_game))]
#[derive(Debug, Clone)]
struct DeleteGameTimer {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,
    scheduled_at: spacetimedb::ScheduleAt,
    #[unique]
    game_id: u32,
}

/////// FUNCTIONS

fn get_board(ctx: &ReducerContext, g: &Game) -> Result<[u8; 9], String> {
    let mut board: [u8; 9] = [0; 9];
    for (i, gm) in ctx.db.game_move().game_id().filter(&g.id).enumerate() {
        let who = if i % 2 == 0 { 1 } else { 2 };
        let index: usize = gm.position as usize;
        board[index] = who;
    }

    Ok(board)
}

fn filled_cells(board: [u8; 9]) -> u8 {
    let mut count = 0;
    for v in board {
        if v > 0 {
            count += 1;
        }
    }
    count
}

fn is_full(board: [u8; 9]) -> bool {
    filled_cells(board) == 9
}

// 0 1 2
// 3 4 5
// 6 7 8
fn has_won(board: [u8; 9], v: u8) -> bool {
    return (board[0] == v && board[1] == v && board[2] == v) // H
        || (board[3] == v && board[4] == v && board[5] == v)
        || (board[6] == v && board[7] == v && board[8] == v)
        || (board[0] == v && board[3] == v && board[6] == v) // V
        || (board[1] == v && board[4] == v && board[7] == v)
        || (board[2] == v && board[5] == v && board[8] == v)
        || (board[0] == v && board[4] == v && board[8] == v) // D
        || (board[2] == v && board[4] == v && board[6] == v);
}

fn give_feedback(ctx: &ReducerContext, game_id: u32, player_id: Identity, msg: String) {
    let row = Feedback {
        id: 0,
        game_id: game_id.to_owned(),
        player_id: player_id.to_owned(),
        when: ctx.timestamp,
        message: msg,
    };
    ctx.db.feedback().insert(row);
}

fn schedule_delete_game(ctx: &ReducerContext, game_id: u32) {
    log::info!("schedule the deletion of game_id {} in 250ms...", game_id);
    _ = ctx.db.delete_game_timer().try_insert(DeleteGameTimer {
        scheduled_id: 0,
        scheduled_at: ScheduleAt::Time(
            ctx.timestamp + TimeDuration::from(Duration::from_secs_f32(0.25)),
        ),
        game_id,
    });
}

fn update_stats(
    ctx: &ReducerContext,
    player_id: Identity,
    starts: u32,
    left_early: u32,
    wins: u32,
    ties: u32,
    losses: u32,
) -> bool {
    if let Some(mut ps) = ctx.db.player_stats().id().find(player_id) {
        ps.starts += starts;
        ps.left_early += left_early;
        ps.wins += wins;
        ps.ties += ties;
        ps.losses += losses;
        ps.updated_at = ctx.timestamp;
        ctx.db.player_stats().id().update(ps);
        return true;
    }
    false
}

/////// REDUCERS

// #[spacetimedb::reducer(init)]
// fn init(ctx: &ReducerContext) {
//     // called at module start
// }

#[spacetimedb::reducer(client_connected)]
fn identity_connected(ctx: &ReducerContext) {
    // called everytime a new client connects
    log::info!("Client {} connected.", ctx.sender);

    let maybe_ps = ctx.db.player_stats().try_insert(PlayerStats {
        id: ctx.sender,
        starts: 1,
        left_early: 0,
        wins: 0,
        ties: 0,
        losses: 0,
        created_at: ctx.timestamp,
        updated_at: ctx.timestamp,
    });
    if maybe_ps.is_err() {
        update_stats(ctx, ctx.sender, 1, 0, 0, 0, 0);
    }

    if let Some(mut ps) = ctx.db.player_stats().id().find(ctx.sender) {
        ps.starts += 1;
        ps.updated_at = ctx.timestamp;
        ctx.db.player_stats().id().update(ps);
    } else {
    }

    let id0: Identity = Identity::ZERO;
    let unstarted_game: Option<Game> = ctx.db.game().p2().filter(&id0).next();

    if let Some(mut g) = unstarted_game {
        g.p2 = ctx.sender;
        g = ctx.db.game().id().update(g);
        log::info!("Game {} starting...", g.id);
        give_feedback(
            ctx,
            g.id,
            g.p1,
            "Game starting! You play first with the Xs.".to_string(),
        );
        give_feedback(
            ctx,
            g.id,
            g.p2,
            "Game starting! Opponent plays first. You play the Os.".to_string(),
        );
    } else {
        let g = Game {
            id: 0,
            p1: ctx.sender,
            p2: Identity::ZERO,
            when: ctx.timestamp,
        };
        let g = ctx.db.game().insert(g);
        log::info!("Game {} created.", g.id);
        give_feedback(
            ctx,
            g.id,
            ctx.sender,
            "Waiting for an opponent to join...".to_string(),
        );
    }
}

#[spacetimedb::reducer(client_disconnected)]
fn identity_disconnected(ctx: &ReducerContext) {
    // called everytime a client disconnects
    log::info!("Client {} disconnected.", ctx.sender);

    update_stats(ctx, ctx.sender, 0, 1, 0, 0, 0);

    let g = ctx.db.game().p1().filter(&ctx.sender).next();
    if let Some(g) = g {
        give_feedback(ctx, g.id, g.p2, "other player left".to_string());
        schedule_delete_game(ctx, g.id);
        return;
    }

    let g = ctx.db.game().p2().filter(&ctx.sender).next();
    if let Some(g) = g {
        give_feedback(ctx, g.id, g.p1, "other player left".to_string());
        schedule_delete_game(ctx, g.id);
        return;
    }
}

#[spacetimedb::reducer]
pub fn play(ctx: &ReducerContext, game_id: u32, position: u8) {
    log::info!(
        "Client {} trying to play in game_id {} the position {}...",
        ctx.sender,
        game_id,
        position
    );

    if let Some(g) = ctx.db.game().id().find(&game_id) {
        let mut board = get_board(ctx, &g).unwrap();
        log::info!("board before move: {:?}", board);

        let p1_to_play_next = filled_cells(board) % 2 == 0;
        let id_to_play_next = if p1_to_play_next { g.p1 } else { g.p2 };

        if ctx.sender != id_to_play_next {
            log::info!("not your turn!");
            give_feedback(ctx, game_id, ctx.sender, "not your turn!".to_string());
            return;
        }

        if is_full(board) {
            log::info!("trying to play on a full board!");
            give_feedback(
                ctx,
                game_id,
                ctx.sender,
                "trying to play on a full board!".to_string(),
            );
            return;
        }

        let p_size: usize = position as usize;
        if board[p_size] != 0 {
            log::info!("trying to play on a non-empty cell!");
            give_feedback(
                ctx,
                game_id,
                ctx.sender,
                "trying to play on a non-empty cell!".to_string(),
            );
            return;
        }

        log::info!("Move goes through...");
        let v: u8 = if g.p1 == ctx.sender { 1 } else { 2 };
        board[p_size] = v;
        log::info!("board after move:  {:?}", board);

        let fb_s = if v == 1 {
            "Valid move from X!".to_string()
        } else {
            "Valid move from O!".to_string()
        };
        give_feedback(ctx, game_id, g.p1, fb_s.to_owned());
        give_feedback(ctx, game_id, g.p2, fb_s);

        let mv = GameMove {
            id: 0,
            game_id: game_id.to_owned(),
            player_id: ctx.sender,
            when: ctx.timestamp,
            position: position.to_owned(),
        };
        ctx.db.game_move().insert(mv);

        if has_won(board, v) {
            let winner = ctx.sender;
            let loser = if g.p1 == ctx.sender { g.p2 } else { g.p1 };
            update_stats(ctx, winner, 0, 0, 1, 0, 0);
            update_stats(ctx, loser, 0, 0, 0, 0, 1);
            log::info!("Player {} won against {}.", winner, loser);
            give_feedback(ctx, game_id, winner, "you won!".to_string());
            give_feedback(ctx, game_id, loser, "you lost!".to_string());
        } else if is_full(board) {
            log::info!("Players tied.");
            let fb_s = "the game is a tie.".to_string();
            update_stats(ctx, g.p1, 0, 0, 0, 1, 0);
            update_stats(ctx, g.p2, 0, 0, 0, 1, 0);
            give_feedback(ctx, game_id, g.p1, fb_s.to_owned());
            give_feedback(ctx, game_id, g.p2, fb_s);
        } else {
            log::info!("Non-finishing move...");
            return;
        }

        ctx.db.game().id().update(g.to_owned());
        schedule_delete_game(ctx, g.id);
    } else {
        log::info!("game not found");
        give_feedback(
            ctx,
            game_id,
            ctx.sender,
            "Game not found! Must have ended?".to_string(),
        );
    }
}

#[spacetimedb::reducer]
fn delete_game(ctx: &ReducerContext, timer: DeleteGameTimer) {
    let game_id = timer.game_id;
    log::info!("deleting every data related to game_id {}...", game_id);
    ctx.db.game().id().delete(game_id);
    for gm in ctx.db.game_move().game_id().filter(&game_id) {
        ctx.db.game_move().id().delete(gm.id);
    }
    for fb in ctx.db.feedback().game_id().filter(&game_id) {
        ctx.db.feedback().id().delete(fb.id);
    }
}
