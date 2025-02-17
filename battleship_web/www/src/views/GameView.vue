<script setup lang="ts">
import ButtonComponent from '@/components/ButtonComponent.vue';
import GameSquare from '@/components/GameSquare.vue';
import { useGameStore } from '@/stores/game';

const store = useGameStore();
</script>

<template>
  <main class="game-view">
    <h1
      class="game-view__heading"
      data-ui="game-view-heading"
    >
      Battleship
    </h1>
    <p
      class="game-view__description"
      data-ui="game-view-description"
    >
      Sink all the ships to win!
    </p>
    <section class="game-view__game">
      <template v-if="store.isEnd">
        <h2
          class="game-view__subheading"
          data-ui="game-view-subheading"
        >
          You win!
        </h2>
        <div class="game-view__statistics">
          <p
            class="game-view__stat"
            data-ui="game-view-stat-amt-of-turns"
          >
            # of turns: {{ store.amtOfTurns }}
          </p>
          <p
            class="game-view__stat"
            data-ui="game-view-stat-amt-of-hits"
          >
            # of hits: {{ store.amtOfHits }}
          </p>
          <p
            class="game-view__stat"
            data-ui="game-view-stat-amt-of-misses"
          >
            # of misses: {{ store.amtOfMisses }}
          </p>
          <p
            class="game-view__stat"
            data-ui="game-view-stat-ships-sunk"
          >
            # of ships sunk: {{ store.shipsSunk }}
          </p>
        </div>
        <ButtonComponent @click="store.reset">Play again</ButtonComponent>
      </template>
      <div
        v-else
        class="game-view__grid"
        data-ui="game-view-grid"
      >
        <GameSquare
          v-for="(cell, idx) in store.board"
          :key="`game-square-${idx}`"
          :variant="cell"
          data-ui="game-view-cell"
          @click="store.shoot(idx)"
        />
      </div>
    </section>
  </main>
</template>

<style lang="scss" scoped>
@use '../styles/tokens.scss' as *;
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

  &__subheading {
    @include text-heading-lg;
  }

  &__description {
    @include text-base;
  }

  &__statistics {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    flex-wrap: wrap;
    gap: $space-one-x;
    max-width: 300px;
  }

  &__stat {
    @include text-base;
  }

  &__subheading,
  &__stat {
    --text-color: #{$color-gray-light};
  }

  &__game {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: $space-two-x;
    background-color: $color-blue-dark;
    padding: $space-one-x;
    width: 600px;
    max-width: 600px;
    height: 600px;
    max-height: 600px;
    border-radius: 8px;
  }

  &__grid {
    display: grid;
    grid-template-columns: repeat(8, 1fr);
    gap: $space-half-x;
  }
}
</style>
