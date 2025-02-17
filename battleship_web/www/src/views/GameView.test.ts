import { nextTick } from 'vue';
import { describe, test, expect, vi, beforeEach } from 'vitest';
import { createTestingPinia } from '@pinia/testing';
import { mount } from '@vue/test-utils';
import { useGameStore } from '@/stores/game';
import { battleshipWeb } from '@/wasm';
import GameView from './GameView.vue';

vi.mock('@/wasm', () => ({
  battleshipWeb: {
    amt_of_turns: vi.fn(() => 0),
    amt_of_hits: vi.fn(() => 0),
    amt_of_misses: vi.fn(() => 0),
    ships_sunk: vi.fn(() => 0),
    is_end: vi.fn(() => false),
    shoot: vi.fn(),
    play_again: vi.fn(),
  },
  getGameBoard: vi.fn(() => new Array(64).fill('unknown')),
}));

describe('<GameView />', () => {
  const wrapper = mount(GameView, {
    global: {
      plugins: [createTestingPinia({ createSpy: vi.fn, stubActions: false })],
    },
  });

  const store = useGameStore();

  beforeEach(async () => {
    store.isEnd = false;
    await nextTick();
  });

  test('game start view', () => {
    const headingText = wrapper.find('[data-ui="game-view-heading"]').text();
    const descriptionText = wrapper.find('[data-ui="game-view-description"]').text();
    const gridExists = wrapper.find('[data-ui="game-view-grid"]').exists();
    const gridCellAmount = wrapper.findAll('[data-ui="game-view-grid-cell"]').length;
    expect(headingText).toEqual('Battleship');
    expect(descriptionText).toEqual('Sink all the ships to win!');
    expect(gridExists).toBeTruthy();
    expect(gridCellAmount).toEqual(64);
  });

  test('game end view', async () => {
    store.isEnd = true;
    await nextTick();

    const subheadingText = wrapper.find('[data-ui="game-view-subheading"]').text();
    const amtOfTurnsText = wrapper.find('[data-ui="game-view-stat-amt-of-turns"]').text();
    const amtOfHitsText = wrapper.find('[data-ui="game-view-stat-amt-of-hits"]').text();
    const amtOfMissesText = wrapper.find('[data-ui="game-view-stat-amt-of-misses"]').text();
    const shipsSunkText = wrapper.find('[data-ui="game-view-stat-ships-sunk"]').text();
    const playAgainButtonExists = wrapper.find('[data-ui="game-view-play-again"]').exists();
    expect(subheadingText).toEqual('You win!');
    expect(amtOfTurnsText).toEqual('# of turns: 0');
    expect(amtOfHitsText).toEqual('# of hits: 0');
    expect(amtOfMissesText).toEqual('# of misses: 0');
    expect(shipsSunkText).toEqual('# of ships sunk: 0');
    expect(playAgainButtonExists).toBeTruthy();
  });

  test('game cell click', async () => {
    const gameCell = wrapper.find('[data-ui="game-view-grid-cell"]');
    const shootSpy = vi.spyOn(battleshipWeb, 'shoot');
    await gameCell.trigger('click');
    expect(shootSpy).toHaveBeenCalledOnce();
    expect(shootSpy).toHaveBeenCalledWith(0);
  });

  test('play again click', async () => {
    store.isEnd = true;
    await nextTick();

    const playAgainButton = wrapper.find('[data-ui="game-view-play-again"]');
    const playAgainSpy = vi.spyOn(battleshipWeb, 'play_again');
    await playAgainButton.trigger('click');
    expect(playAgainSpy).toHaveBeenCalledOnce();
  });
});
