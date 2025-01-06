<script setup lang="ts">
import GameSquare from './GameSquare.vue';
</script>

<template>
  <section class="game-key">
    <h2 class="game-key__heading">Key</h2>
    <p class="game-key__pair">
      Unknown
      <GameSquare
        mini
        disabled
      />
    </p>
    <p class="game-key__pair">
      Hit
      <GameSquare
        variant="hit"
        mini
        disabled
      />
    </p>
    <p class="game-key__pair">
      Miss
      <GameSquare
        variant="miss"
        mini
        disabled
      />
    </p>
  </section>
</template>

<style lang="scss" scoped>
@use '../styles/tokens.scss' as *;
@use '../styles/text.scss' as *;

.game-key {
  display: flex;
  flex-direction: column;
  gap: $space-half-x;
  padding: $space-one-x;
  background-color: $color-blue-dark;
  --text-color: #{$color-gray-light};

  &__heading {
    @include text-heading-card;
  }

  &__pair {
    @include text-base;
    display: flex;
    flex-direction: row;
    gap: $space-one-x;
    justify-content: space-between;
    align-items: center;
  }
}
</style>
