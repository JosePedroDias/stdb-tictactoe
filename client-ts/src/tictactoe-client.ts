import { Identity } from '@clockworklabs/spacetimedb-sdk';
import { DbConnection, ErrorContext, EventContext, Game, GameMove, Feedback, PlayerStats } from './module_bindings';

import { updateBoard, updateFeedback, updateNextPlayer } from './ui.js';

function toMark(v: number) {
  return (v == 1 ? 'X' : v == 2 ? 'O' : ' ');
}

const IDENTITY_ZERO = BigInt(0);

export type Board = Array<number>;

type Unsubscribable = { unsubscribe(): void };

export class TicTacToeClient {
  conn: DbConnection;
  //subsBuilder: Subscrito
  myIdentity?: Identity;
  myIdentityHex?: string;
  myToken?: string;
  volatileHandles: Array<Unsubscribable> = [];

  // these are game-related
  gameId?: number;
  playingFirst: boolean = false;
  board: number[] = new Array(9).fill(0);

  constructor(url: string = 'ws://localhost:3000') {
    this.conn = DbConnection.builder()
      .withUri(url)
      .withModuleName('tic-tac-toe')
      .withToken(localStorage.getItem('auth_token') || '')
      .onConnect(this.onConnect.bind(this))
      .onDisconnect(this.onDisconnect.bind(this))
      .onConnectError(this.onConnectError.bind(this))
      .build();
  }

  play(position: number) {
    this.conn.reducers.play(this.gameId || 0, position);
  }

  private onConnect(
    conn: DbConnection,
    identity: Identity,
    token: string
  ) {
    const idHex = identity.toHexString();
    this.myIdentity = identity;
    this.myIdentityHex = idHex;
    
    console.log('My identity:', idHex);
    

    localStorage.setItem('auth_token', token);
    this.myToken = token;

    console.log(`Connected to stdb w/ identity: ${idHex}`);

    conn
      .subscriptionBuilder()
      .subscribe(`SELECT * FROM game WHERE p1='${idHex}' OR p2='${idHex}'`);

      conn
      .subscriptionBuilder()
      .subscribe(`SELECT * FROM player_stats WHERE id='${idHex}'`);

    conn.db.game.onInsert((_ctx: EventContext, game: Game) => this.onGameInsert(game));
    conn.db.game.onUpdate((_ctx: EventContext, gameOld: Game, game: Game) => this.onGameUpdate(gameOld, game));
    conn.db.game.onDelete((_ctx: EventContext, game: Game) => this.onGameDelete(game));
    conn.db.playerStats.onInsert((_ctx: EventContext, ps: PlayerStats) => this.onPlayerStatsInsert(ps));
    conn.db.playerStats.onUpdate((_ctx: EventContext, psOld: PlayerStats, ps: PlayerStats) => this.onPlayerStatsUpdate(psOld, ps));
    conn.db.gameMove.onInsert((_ctx: EventContext, gm: GameMove) => this.onGameMoveInsert(gm));
    conn.db.feedback.onInsert((_ctx: EventContext, fb: Feedback) => this.onFeedbackInsert(fb));
  }

  private onDisconnect() {
    console.log('Disconnected from stdb');
  }

  private onConnectError(_ctx: ErrorContext, err: Error) {
    console.log('Error connecting to stdb:', err);
  }

  private setupOnceGameIdIsSet() {
    const h1 = this.conn
    .subscriptionBuilder()
    .subscribe(`SELECT * FROM game_move WHERE game_id=${this.gameId}`);

    const h2 = this.conn
    .subscriptionBuilder()
    .subscribe(`SELECT * FROM feedback WHERE game_id=${this.gameId} AND player_id='${this.myIdentityHex}'`);

    this.volatileHandles.push(h1);
    this.volatileHandles.push(h2);

    console.log(`Setting up for gameId: ${this.gameId}`);
  }

  private onGameInsert(game: Game) {
    console.log('Game inserted:', game);

    if (this.gameId && game.id === this.gameId) {
      return;
    }

    this.gameId = game.id;
    this.playingFirst = game.p1 === this.myIdentity;
    this.setupOnceGameIdIsSet();
  }

  private onGameUpdate(gameOld: Game, game: Game) {
    console.log('Game updated:', game);//, 'from old:', gameOld);

    if (this.gameId) {
      return;
    } else {
      this.gameId = game.id;
      this.playingFirst = game.p1 === this.myIdentity;
      this.setupOnceGameIdIsSet();
    }

    if (game.p2.__identity__ !== IDENTITY_ZERO && gameOld.p2.__identity__ === IDENTITY_ZERO) {
      const msg = `Game started! ${this.playingFirst ? 'You play first with the Xs!' : 'Opponent plays first... You play Os.'}`;
      updateFeedback(msg);
    }
  }

  private onGameDelete(game: Game) {
    console.log('Game deleted:', game);
    this.gameId = undefined;

    for (let h of this.volatileHandles) h.unsubscribe();
    this.volatileHandles = [];

    // auto-clicking new game seems to fail. for now needs a browser refresh
    //setTimeout(this.conn.reducers.newGame, 250 + Math.random() * 500);
  }

  private onPlayerStatsInsert(ps: PlayerStats) {
    console.log('Player stats inserted:', ps);
  }

  private onPlayerStatsUpdate(_psOld: PlayerStats, ps: PlayerStats) {
    console.log('Player stats updated:', ps);//, 'from old:', psOld);
  }

  private onGameMoveInsert(gm: GameMove) {
    console.log('Game move inserted:', gm);
    const nonZeros = this.board.filter((v) => v !== 0).length;
    const v = nonZeros % 2 === 0 ? 1 : 2;
    this.board[gm.position] = v;
    console.log('Board:', this.board);
    updateBoard(this.board);
    updateNextPlayer(`Next player: ${toMark(v === 1 ? 2 : 1)}.`);
  }

  private onFeedbackInsert(fb: Feedback) {
    console.log('Feedback inserted:', fb);
    updateFeedback(fb.message);
  }
}