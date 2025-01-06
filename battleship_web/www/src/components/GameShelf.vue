<script setup lang="ts">
import GameSquare from './GameSquare.vue';
</script>

<template>
  <section class="game-shelf">
    <GameSquare
      v-for="i in 64"
      :key="`game-square-${i}`"
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
