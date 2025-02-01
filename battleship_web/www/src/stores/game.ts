import { ref } from 'vue';
import { defineStore } from 'pinia';
import { battleshipWeb, getGameBoard, resetGame } from '@/utils/wasm';

export const useGameStore = defineStore('game', () => {
  const board = ref(getGameBoard(64));
  const isEnd = ref(false);

  const amtOfTurns = ref(battleshipWeb.amt_of_turns());
  const amtOfHits = ref(battleshipWeb.amt_of_hits());
  const amtOfMisses = ref(battleshipWeb.amt_of_misses());
  const shipsSunk = ref(battleshipWeb.ships_sunk());

  const setBoardState = () => {
    board.value = getGameBoard(64);
    amtOfTurns.value = battleshipWeb.amt_of_turns();
    amtOfHits.value = battleshipWeb.amt_of_hits();
    amtOfMisses.value = battleshipWeb.amt_of_misses();
    shipsSunk.value = battleshipWeb.ships_sunk();
    isEnd.value = battleshipWeb.is_end();
  };

  const shoot = (idx: number) => {
    battleshipWeb.shoot(idx);
    setBoardState();
  };

  const reset = () => {
    resetGame();
    setBoardState();
  };

  return {
    board,
    amtOfTurns,
    amtOfHits,
    amtOfMisses,
    shipsSunk,
    isEnd,
    shoot,
    reset,
  };
});
