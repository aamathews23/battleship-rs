<script setup lang="ts">
import { computed } from 'vue';
import type { GameSquareVariant } from '@/types';

export interface GameSquareProps {
  variant?: GameSquareVariant;
}

const props = withDefaults(defineProps<GameSquareProps>(), {
  variant: 'unknown',
});

const classes = computed(() => ['game-square', `game-square--${props.variant}`]);

const ariaLabel = computed(() => {
  switch (props.variant) {
    case 'miss':
      return 'Miss...';
    case 'hit':
      return 'Hit!';
    case 'ship':
      return 'Ship.';
    default:
      return 'Unknown.';
  }
});
</script>

<template>
  <button
    :class="classes"
    :aria-label="ariaLabel"
  />
</template>

<style lang="scss" scoped>
@use '../styles/tokens.scss' as *;

.game-square {
  width: $space-four-x;
  height: $space-four-x;
  outline: none;
  border: none;
  background-color: var(--game-square-bg-color);
  transition: all 300ms;
  border-radius: 4px;

  &:hover {
    cursor: pointer;
  }

  &:hover,
  &:focus {
    border: 1px solid $color-gray-dark;
  }

  &--unknown {
    --game-square-bg-color: #{$color-blue-light};

    &:hover,
    &:focus {
      --game-square-bg-color: #{$color-blue};
    }
  }

  &--ship {
    --game-square-bg-color: #{$color-orange-light};

    &:hover,
    &:focus {
      --game-square-bg-color: #{$color-orange};
    }
  }

  &--hit {
    --game-square-bg-color: #{$color-green-light};
  }

  &--miss {
    --game-square-bg-color: #{$color-red-light};
  }
}
</style>
