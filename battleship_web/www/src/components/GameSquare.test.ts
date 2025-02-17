import { describe, test, expect, vi } from 'vitest';
import { shallowMount } from '@vue/test-utils';
import GameSquare from './GameSquare.vue';
import { type GameSquareVariant } from '@/types';

const onClick = vi.fn();

describe('<GameSquare />', () => {
  const getWrapper = (variant: GameSquareVariant = 'unknown') =>
    shallowMount(GameSquare, {
      props: {
        variant,
      },
      attrs: {
        onClick,
      },
    });

  describe('variants', () => {
    test('defaults to the unknown variant', () => {
      const wrapper = getWrapper();
      expect(wrapper.classes()).toContain('game-square--unknown');
      expect(wrapper.attributes()['aria-label']).toEqual('Unknown.');
    });
    test('renders the hit variant', () => {
      const wrapper = getWrapper('hit');
      expect(wrapper.classes()).toContain('game-square--hit');
      expect(wrapper.attributes()['aria-label']).toEqual('Hit!');
    });
    test('renders the miss variant', () => {
      const wrapper = getWrapper('miss');
      expect(wrapper.classes()).toContain('game-square--miss');
      expect(wrapper.attributes()['aria-label']).toEqual('Miss...');
    });
    test('renders the ship variant', () => {
      const wrapper = getWrapper('ship');
      expect(wrapper.classes()).toContain('game-square--ship');
      expect(wrapper.attributes()['aria-label']).toEqual('Ship.');
    });
  });
  describe('functionality', () => {
    test('click event handler fires when button is clicked', async () => {
      const wrapper = getWrapper();
      await wrapper.trigger('click');
      expect(onClick).toHaveBeenCalled();
    });
  });
});
