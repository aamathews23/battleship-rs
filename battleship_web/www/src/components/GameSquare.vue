<script setup lang="ts">
import { computed } from 'vue';
import type { GameSquareVariant } from '@/types';

export interface GameSquareProps {
  variant?: GameSquareVariant;
  mini?: boolean;
  disabled?: boolean;
}

const props = withDefaults(defineProps<GameSquareProps>(), {
  variant: 'unknown',
  mini: false,
  disabled: false,
});

const tag = props.mini ? 'div' : 'button';

const attrs = props.mini ? { 'aria-hidden': true } : { type: 'button' };

const classes = computed(() => ({
  'game-square': true,
  [`game-square--${props.variant}`]: true,
  'game-square--mini': props.mini,
  'game-square--disabled': props.disabled,
}));

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
  <component
    :is="tag"
    :class="classes"
    :aria-label="ariaLabel"
    v-bind="attrs"
  />
</template>

<style lang="scss" scoped>
@use '../styles/tokens.scss' as *;

.game-square {
  width: $space-four-x;
  height: $space-four-x;
  outline: none;
  border: none;

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

  &--ship {
    background-color: $color-orange-light;
    transition: all 300ms;

    &:hover,
    &:focus {
      background-color: $color-orange;
      cursor: pointer;
    }

    &.game-square--disabled {
      &:hover,
      &:focus {
        background-color: $color-orange-light;
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
