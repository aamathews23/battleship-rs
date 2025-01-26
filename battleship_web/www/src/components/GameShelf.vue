<script setup lang="ts">
import GameSquare from './GameSquare.vue';
import { useGameStore } from '@/stores/game';

const store = useGameStore();
</script>

<template>
  <section class="game-shelf">
    <GameSquare
      v-for="(cell, idx) in store.board"
      :key="`game-square-${idx}`"
      :variant="cell"
      @click="store.shoot(idx)"
    />
  </section>
</template>

<style lang="scss" scoped>
@use '../styles/tokens.scss' as *;

.game-shelf {
  display: grid;
  grid-template-columns: repeat(8, 1fr);
  gap: $space-half-x;
  background-color: $color-blue-dark;
  padding: $space-one-x;
  max-width: 600px;
  max-height: 600px;
}
</style>
