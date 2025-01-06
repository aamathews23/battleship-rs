<template>
  <section class="game-statistics">
    <p class="game-statistics__stat"># of turns: 0</p>
    <p class="game-statistics__stat"># of hits: 0</p>
    <p class="game-statistics__stat"># of misses: 0</p>
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
