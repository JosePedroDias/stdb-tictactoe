import { Identity } from '@clockworklabs/spacetimedb-sdk';
import { DbConnection, ErrorContext, EventContext, Game, GameMove, Feedback } from './module_bindings';

import { updateBoard, updateFeedback, updateNextPlayer } from './ui.js';

function toMark(v: number) {
  return (v == 1 ? 'X' : v == 2 ? 'O' : ' ');
}

const GR_UNSTARTED = 0;
const GR_ONGOING = 1;
/*const GR_P1_WON = 2;
const GR_TIE = 3;
const GR_P2_WON = 4;
const GR_ABANDONED = 5;*/

//updateFeedback('waiting for opponent');

export type Board = Array<number>;

export class TicTacToeClient {
  conn: DbConnection;
  //subsBuilder: Subscrito
  myIdentity?: Identity;
  myIdentityHex?: string;
  myToken?: string;
  board: number[] = new Array(9).fill(0);

  // these are game-related
  gameId?: number;
  playingFirst: boolean = false;

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

    conn.db.game.onInsert((_ctx: EventContext, game: Game) => this.onGameInsert(game));
    conn.db.game.onUpdate((_ctx: EventContext, gameOld: Game, game: Game) => this.onGameUpdate(gameOld, game));
    conn.db.gameMove.onInsert((_ctx: EventContext, gm: GameMove) => this.onGameMoveInsert(gm));
    conn.db.feedback.onInsert((_ctx: EventContext, fb: Feedback) => this.onFeedbackInsert(fb));

    conn.reducers.ready();
  }

  private onDisconnect() {
    console.log('Disconnected from stdb');
  }

  private onConnectError(_ctx: ErrorContext, err: Error) {
    console.log('Error connecting to stdb:', err);
  }

  private setupOnceGameIdIsSet() {
    this.conn
    .subscriptionBuilder()
    .subscribe(`SELECT * FROM game_move WHERE game_id=${this.gameId}'`);

    this.conn
    .subscriptionBuilder()
    .subscribe(`SELECT * FROM feedback WHERE game_id=${this.gameId} AND player_id='${this.myIdentityHex}'`);

    console.log(`Setting up for gameId: ${this.gameId}`);
  }

  private onGameInsert(game: Game) {
    console.log('Game inserted:', game);

    if (this.gameId) {
      console.log('Game already set up, ignoring insert');
      return;
    }

    this.gameId = game.id;
    this.playingFirst = game.p1 === this.myIdentity;
    this.setupOnceGameIdIsSet();
  }

  private onGameUpdate(gameOld: Game, game: Game) {
    console.log('Game updated:', game, 'from old:', gameOld);

    if (this.gameId) {
      console.log('Game already set up, ignoring update setup');
      return;
    } else {
      this.gameId = game.id;
      this.playingFirst = game.p1 === this.myIdentity;
      this.setupOnceGameIdIsSet();
    }

    if (game.result === GR_ONGOING && gameOld.result == GR_UNSTARTED) {
      const msg = `Game started! ${this.playingFirst ? 'You play first with the Xs!' : 'Opponent plays first... You play Os.'}`;
      updateFeedback(msg);
    }
  }

  private onGameMoveInsert(gm: GameMove) {
    console.log('Game move inserted:', gm);
    const nonZeros = this.board.filter((v) => v !== 0).length;
    const v = nonZeros % 2 === 0 ? 1 : 2;
    this.board[gm.position] = v;
    console.log('Board:', this.board);
    updateBoard(this.board);
    updateNextPlayer(`Next player: ${toMark(v === 1 ? 2 : 1)}`);
  }

  private onFeedbackInsert(fb: Feedback) {
    console.log('Feedback inserted:', fb);
    updateFeedback(fb.message);
  }
}