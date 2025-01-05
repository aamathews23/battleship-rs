<script setup lang="ts">
import { useRouter } from 'vue-router';

const router = useRouter();

const handleOnClick = () => {
  router.push('game');
};
</script>

<template>
  <main>
    <h1>Battleship</h1>
    <p>The classic game of Battleship created in Rust, Vue and Wasm.</p>
    <button @click="handleOnClick">Play game</button>
  </main>
</template>
