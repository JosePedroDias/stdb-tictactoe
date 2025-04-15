import { TicTacToeClient } from './tictactoe-client';
import { setup } from './ui';

const cli = new TicTacToeClient();
setup((n: number) => cli.play(n));
