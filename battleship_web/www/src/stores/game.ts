import { ref } from 'vue';
import { defineStore } from 'pinia';
import { battleshipWeb, getGameBoard } from '@/utils/wasm';

export const useGameStore = defineStore('game', () => {
  const board = ref(getGameBoard(64));

  const amtOfTurns = ref(battleshipWeb.amt_of_turns());
  const amtOfHits = ref(battleshipWeb.amt_of_hits());
  const amtOfMisses = ref(battleshipWeb.amt_of_misses());
  const shipsSunk = ref(battleshipWeb.ships_sunk());

  const shoot = (idx: number) => {
    battleshipWeb.shoot(idx);
    board.value = getGameBoard(64);
    amtOfTurns.value = battleshipWeb.amt_of_turns();
    amtOfHits.value = battleshipWeb.amt_of_hits();
    amtOfMisses.value = battleshipWeb.amt_of_misses();
    shipsSunk.value = battleshipWeb.ships_sunk();
  };

  return {
    board,
    amtOfTurns,
    amtOfHits,
    amtOfMisses,
    shipsSunk,
    shoot,
  };
});
