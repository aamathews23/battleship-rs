<script setup lang="ts">
import GameShelf from '@/components/GameShelf.vue';
import GameKey from '@/components/GameKey.vue';
import GameStatistics from '@/components/GameStatistics.vue';
import ButtonComponent from '@/components/ButtonComponent.vue';
import { useGameStore } from '@/stores/game';

const store = useGameStore();
</script>

<template>
  <main class="game-view">
    <h1 class="game-view__heading">Battleship</h1>
    <p class="game-view__description">Sink all the ships to win!</p>
    <section class="game-view__game">
      <div class="game-view__controls">
        <GameKey />
        <GameStatistics />
        <ButtonComponent
          variant="secondary"
          @click="store.reset"
        >
          Reset
        </ButtonComponent>
      </div>
      <GameShelf />
    </section>
  </main>
</template>

<style lang="scss" scoped>
@use '../styles/text.scss' as *;

.game-view {
  display: flex;
  flex-direction: column;
  gap: 32px;
  justify-content: center;
  align-items: center;
  min-height: 100vh;

  &__heading {
    @include text-heading-xl;
  }

  &__description {
    @include text-base;
  }

  &__game {
    display: grid;
    grid-template-columns: 1fr 600px;
    gap: 64px;
  }

  &__controls {
    display: flex;
    flex-direction: column;
    gap: 32px;
  }
}
</style>
