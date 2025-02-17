import { describe, test, expect, beforeEach, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import router from '@/router';
import HomeView from './HomeView.vue';

describe('<HomeView />', () => {
  beforeEach(async () => {
    router.push('/');
    await router.isReady();
  });

  const wrapper = mount(HomeView, {
    global: {
      plugins: [router],
    },
  });

  test('has the title', () => {
    const heading = wrapper.find('[data-ui="home-view-heading"]').text();
    expect(heading).toEqual('Battleship');
  });
  test('has the description', () => {
    const description = wrapper.find('[data-ui="home-view-description"]').text();
    expect(description).toEqual('The classic game of Battleship created in Rust, Vue and Wasm.');
  });
  test('routes to the game page', async () => {
    const push = vi.spyOn(router, 'push');
    const cta = wrapper.find('[data-ui="home-view-cta"]');
    await cta.trigger('click');

    expect(push).toHaveBeenCalledOnce();
    expect(push).toHaveBeenCalledWith('game');
  });
});
