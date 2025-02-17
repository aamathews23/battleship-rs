<script setup lang="ts">
import { computed } from 'vue';

export interface ButtonComponentProps {
  class?: string;
  type?: 'button' | 'submit' | 'reset';
  variant?: 'primary' | 'secondary';
  dataUi?: string;
}

const props = withDefaults(defineProps<ButtonComponentProps>(), {
  class: undefined,
  type: 'button',
  variant: 'primary',
  dataUi: 'button',
});

const classes = computed(() => ({
  button: true,
  [`button--${props.variant}`]: true,
  [`${props.class}`]: !!props.class,
}));
</script>

<template>
  <button
    :class="classes"
    :type="type"
    :data-ui="dataUi"
  >
    <slot />
  </button>
</template>

<style lang="scss" scoped>
@use '../styles/tokens.scss' as *;
@use '../styles/text.scss' as *;

.button {
  @include text-base;
  min-width: 8rem;
  height: 2.5rem;
  border-width: 2px;
  border-style: solid;
  border-radius: $space-half-x;
  transition: all 300ms;
  outline: none;

  &:hover {
    cursor: pointer;
  }

  &--primary {
    color: $color-gray-light;
    background-color: $color-gray-primary;
    border-color: $color-gray-primary;

    &:hover,
    &:focus {
      background-color: $color-gray-primary-dark;
      border-color: $color-gray-primary-dark;
    }
  }

  &--secondary {
    color: $color-gray-primary;
    background-color: $color-gray-light;
    border-color: $color-gray-primary;

    &:hover,
    &:focus {
      color: $color-gray-light;
      background-color: $color-gray-primary;
      border-color: $color-gray-primary;
    }
  }
}
</style>
