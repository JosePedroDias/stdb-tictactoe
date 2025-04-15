import { Board } from './tictactoe-client';

export function setup(play: any) {
  const boardEl = document.body.querySelector('.board') as Element;
  boardEl.addEventListener('click', (ev) => {
    const el = ev.target as Element;
    // @ts-ignore
    const num = parseInt(el.dataset.num, 10);
    play(num);
  });
}

export function updateFeedback(text: string) {
  const el = document.body.querySelector('.feedback') as Element;
  el.innerHTML = text;
}

export function updateNextPlayer(text: string) {
  const el = document.body.querySelector('.next-player') as Element;
  el.innerHTML = text;
}

export function updateBoard(board: Board) {
  const cellEls = Array.from(document.body.querySelectorAll('.cell'));
  cellEls.forEach((el, i) => {
    el.classList.remove('x');
    el.classList.remove('o');
    const v = board[i];
    if (v === 1) el.classList.add('x');
    else if (v === 2) el.classList.add('o');
  });
}
