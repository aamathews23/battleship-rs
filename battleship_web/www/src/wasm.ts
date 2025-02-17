import * as BattleshipWebWasm from 'battleship_web';
import type { GameSquareVariant, WasmMemory } from '@/types';
import { createBoard } from '@/utils/createBoard';

const memory: WasmMemory = BattleshipWebWasm.wasm_memory();
export const battleshipWeb = BattleshipWebWasm.BattleshipWeb.new();

export const getGameBoard = (size: number): GameSquareVariant[] => {
  const board = new Uint8Array(memory.buffer, battleshipWeb.board(), size);
  return createBoard(board);
};
