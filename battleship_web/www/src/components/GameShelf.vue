<script setup lang="ts">
import GameSquare from './GameSquare.vue';
import { useGameStore } from '@/stores/game';

const store = useGameStore();
</script>

<template>
  <section class="game-shelf">
    <template v-if="store.isEnd">
      <h2 class="game-shelf__heading">You win!</h2>
      <p class="game-shelf__description">Press reset to play again.</p>
    </template>
    <div
      v-else
      class="game-shelf__grid"
    >
      <GameSquare
        v-for="(cell, idx) in store.board"
        :key="`game-square-${idx}`"
        :variant="cell"
        @click="store.shoot(idx)"
      />
    </div>
  </section>
</template>

<style lang="scss" scoped>
@use '../styles/tokens.scss' as *;
@use '../styles/text.scss' as *;

.game-shelf {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: $space-one-x;
  background-color: $color-blue-dark;
  padding: $space-one-x;
  max-width: 600px;
  height: 600px;
  max-height: 600px;

  &__heading {
    @include text-heading-lg;
    --text-color: #{$color-gray-light};
  }

  &__description {
    @include text-base;
    --text-color: #{$color-gray-light};
  }

  &__grid {
    display: grid;
    grid-template-columns: repeat(8, 1fr);
    gap: $space-half-x;
  }
}
</style>
