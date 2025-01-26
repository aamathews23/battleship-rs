<script setup lang="ts">
import { useGameStore } from '@/stores/game';

const store = useGameStore();
</script>

<template>
  <section class="game-statistics">
    <p class="game-statistics__stat"># of turns: {{ store.amtOfTurns }}</p>
    <p class="game-statistics__stat"># of hits: {{ store.amtOfHits }}</p>
    <p class="game-statistics__stat"># of misses: {{ store.amtOfMisses }}</p>
    <p class="game-statistics__stat"># of ships sunk: {{ store.shipsSunk }}</p>
  </section>
</template>

<style lang="scss" scoped>
@use '../styles/tokens.scss' as *;
@use '../styles/text.scss' as *;

.game-statistics {
  display: flex;
  flex-direction: column;
  gap: $space-two-x;

  &__stat {
    @include text-base;
  }
}
</style>
