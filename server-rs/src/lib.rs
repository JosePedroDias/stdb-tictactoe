use spacetimedb::{Identity, ReducerContext, Table, Timestamp};

/////// ENUMS, STRUCTS, CONSTS

const GR_UNSTARTED: u8 = 0;
const GR_ONGOING: u8 = 1;
const GR_P1_WON: u8 = 2;
const GR_TIE: u8 = 3;
const GR_P2_WON: u8 = 4;
const GR_ABANDONED: u8 = 5;

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
    #[index(btree)]
    result: u8,
    when: Timestamp,
    ready1: bool,
    ready2: bool,
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

fn is_full(board: [u8; 9]) -> bool {
    for v in board {
        if v == 0 {
            return false;
        }
    }
    true
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

//fn delete_game(ctx: &ReducerContext, game_id: u32) {
// TODO
//}

/////// REDUCERS

// #[spacetimedb::reducer(init)]
// fn init(ctx: &ReducerContext) {
//     // called at module start
// }

#[spacetimedb::reducer(client_connected)]
fn identity_connected(ctx: &ReducerContext) {
    // called everytime a new client connects
    log::info!("{} connected.", ctx.sender);

    let gr: u8 = 0;
    let unstarted_game: Option<Game> = ctx.db.game().result().filter(&gr).next();

    if let Some(mut g) = unstarted_game {
        g.p2 = ctx.sender;
        g.ready2 = true;
        g.result = GR_ONGOING;
        g = ctx.db.game().id().update(g);
        log::info!("game {} now has 2 players...", g.id);
    } else {
        let g = Game {
            id: 0,
            p1: ctx.sender,
            p2: Identity::ZERO,
            result: GR_UNSTARTED,
            when: ctx.timestamp,
            ready1: true,
            ready2: false,
        };
        let g = ctx.db.game().insert(g);
        log::info!("created game {} connected.", g.id);
    }
}

#[spacetimedb::reducer(client_disconnected)]
fn identity_disconnected(ctx: &ReducerContext) {
    // called everytime a client disconnects
    log::info!("{} disconnected.", ctx.sender);

    let g = ctx.db.game().p1().filter(&ctx.sender).next();
    if let Some(mut g) = g {
        g.ready1 = false;
        g.result = GR_ABANDONED;
        give_feedback(ctx, g.id, g.p2, "other player left".to_string());
        return;
    }

    let g = ctx.db.game().p2().filter(&ctx.sender).next();
    if let Some(mut g) = g {
        g.ready2 = false;
        g.result = GR_ABANDONED;
        give_feedback(ctx, g.id, g.p1, "other player left".to_string());
        return;
    }

    // TODO: delete the game, its moves and feedback after n seconds via a timer
}

#[spacetimedb::reducer]
pub fn ready(ctx: &ReducerContext) {
    log::info!("received ready from {}.", ctx.sender);
    // let g = ctx.db.game().p2().filter(&Identity::ZERO).next();
    // if let Some(mut g) = g {
    //     log::info!("player {} is ready for game {}!", ctx.sender, g.id);
    //     if g.p1 == ctx.sender {
    //         g.ready1 = true;
    //     } else if g.p2 == ctx.sender {
    //         g.ready2 = true;
    //     } else {
    //         panic!("should not happen")
    //     }

    //     if g.ready1 && g.ready2 {
    //         g.result = GR_ONGOING;
    //         log::info!("game {} is starting!", g.id);
    //     }

    //     ctx.db.game().id().update(g);
    // }
}

#[spacetimedb::reducer]
pub fn play(ctx: &ReducerContext, game_id: u32, position: u8) {
    let mut g = ctx
        .db
        .game()
        .id()
        .find(&game_id)
        .ok_or("Game not found")
        .unwrap();

    let mut board = get_board(ctx, &g).unwrap();
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

    let v: u8 = if g.p1 == ctx.sender { 1u8 } else { 2u8 };
    board[p_size] = v;

    let mv = GameMove {
        id: 0,
        game_id: game_id.to_owned(),
        player_id: ctx.sender,
        when: ctx.timestamp,
        position: position.to_owned(),
    };
    ctx.db.game_move().insert(mv);

    if has_won(board, v) {
        g.result = if v == 1 { GR_P1_WON } else { GR_P2_WON };
        give_feedback(ctx, game_id, ctx.sender, "you won!".to_string());
        give_feedback(
            ctx,
            game_id,
            if g.p1 == ctx.sender { g.p2 } else { g.p1 },
            "you lost!".to_string(),
        );
    } else if is_full(board) {
        g.result = GR_TIE;
        give_feedback(ctx, game_id, g.p1, "the game is a tie!".to_string());
        give_feedback(ctx, game_id, g.p2, "the game is a tie!".to_string());
    } else {
        return;
    }

    // TODO: delete the game, its moves and feedback after n seconds via a timer

    ctx.db.game().id().update(g);
}
