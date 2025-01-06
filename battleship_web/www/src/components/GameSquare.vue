<script setup lang="ts">
import { computed } from 'vue';

interface GameSquareProps {
  variant?: 'unknown' | 'hit' | 'miss';
  mini?: boolean;
  disabled?: boolean;
}

const props = withDefaults(defineProps<GameSquareProps>(), {
  variant: 'unknown',
  mini: false,
  disabled: false,
});

const classes = computed(() => ({
  'game-square': true,
  [`game-square--${props.variant}`]: true,
  'game-square--mini': props.mini,
  'game-square--disabled': props.disabled,
}));

const ariaLabel = computed(() => {
  switch (props.variant) {
    case 'unknown':
      return 'Unknown.';
    case 'hit':
      return 'Hit!';
    default:
      return 'Miss...';
  }
});
</script>

<template>
  <div
    :class="classes"
    :aria-label="ariaLabel"
    tabindex="0"
  ></div>
</template>

<style lang="scss" scoped>
@use '../styles/tokens.scss' as *;

.game-square {
  width: $space-four-x;
  height: $space-four-x;
  outline: none;

  &:hover,
  &:focus {
    border: 1px solid $color-gray-dark;

    &.game-square--disabled {
      &:hover {
        border: none;
      }
    }
  }

  &--mini {
    width: $space-one-x;
    height: $space-one-x;
  }

  &--unknown {
    background-color: $color-blue-light;
    transition: all 300ms;

    &:hover,
    &:focus {
      background-color: $color-blue;
      cursor: pointer;
    }

    &.game-square--disabled {
      &:hover,
      &:focus {
        background-color: $color-blue-light;
        cursor: default;
      }
    }
  }

  &--hit {
    background-color: $color-green-light;
  }

  &--miss {
    background-color: $color-red-light;
  }
}
</style>
