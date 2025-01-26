import { BattleshipWeb, wasm_memory } from 'battleship_web';
import type { GameSquareVariant } from '@/types';

type WasmMemory = {
  buffer: ArrayBuffer;
};

const memory: WasmMemory = wasm_memory();
export const battleshipWeb = BattleshipWeb.new();

export const getGameBoard = (size: number): GameSquareVariant[] => {
  const board = new Uint8Array(memory.buffer, battleshipWeb.board(), size);
  const newBoard: GameSquareVariant[] = [];

  for (const cell of board) {
    switch (cell) {
      case 1:
        newBoard.push('miss');
        break;
      case 2:
        newBoard.push('hit');
        break;
      case 3:
        newBoard.push('ship');
        break;
      default:
        newBoard.push('unknown');
        break;
    }
  }

  return newBoard;
};
