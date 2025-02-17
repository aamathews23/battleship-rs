import { CELL_DEBUG } from '@/env';
import type { GameSquareVariant } from '@/types';

export const createBoard = (board: Uint8Array<ArrayBuffer>) => {
  const newBoard: GameSquareVariant[] = [];

  for (const cell of board) {
    if (cell === 1) {
      newBoard.push('miss');
    } else if (cell === 2) {
      newBoard.push('hit');
    } else if (cell === 3 && CELL_DEBUG === 1) {
      newBoard.push('ship');
    } else {
      newBoard.push('unknown');
    }
  }

  return newBoard;
};
