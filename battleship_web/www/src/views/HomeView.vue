<script setup lang="ts">
import ButtonComponent from '@/components/ButtonComponent.vue';
import { useRouter } from 'vue-router';

const router = useRouter();

const handleOnClick = () => {
  router.push('game');
};
</script>

<template>
  <main class="home-view">
    <h1
      class="home-view__heading"
      data-ui="home-view-heading"
    >
      Battleship
    </h1>
    <p
      class="home-view__description"
      data-ui="home-view-description"
    >
      The classic game of Battleship created in Rust, Vue and Wasm.
    </p>
    <ButtonComponent
      data-ui="home-view-cta"
      @click="handleOnClick"
    >
      Play game
    </ButtonComponent>
  </main>
</template>

<style lang="scss" scoped>
@use '../styles/text.scss' as *;

.home-view {
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
}
</style>
