import { describe, test, expect, vi } from 'vitest';
import { shallowMount } from '@vue/test-utils';
import ButtonComponent, { type ButtonComponentProps } from './ButtonComponent.vue';

type ButtonComponentTestWrapperProps = {
  props?: ButtonComponentProps;
  attrs?: Record<string, unknown>;
};

describe('<ButtonComponent />', () => {
  const getWrapper = ({ props, attrs }: ButtonComponentTestWrapperProps = {}) =>
    shallowMount(ButtonComponent, {
      props,
      slots: {
        default: 'Button',
      },
      attrs,
    });

  test('mounts correctly', () => {
    const wrapper = getWrapper();
    expect(wrapper.exists()).toBeTruthy();
    expect(wrapper.text()).toEqual('Button');
  });
  describe('props', () => {
    test('defaults to primary variant', () => {
      const wrapper = getWrapper();
      expect(wrapper.classes()).toContain('button--primary');
    });
    test('has the provided variant', () => {
      const wrapper = getWrapper({ props: { variant: 'secondary' } });
      expect(wrapper.classes()).toContain('button--secondary');
    });
    test('defaults to the button type', () => {
      const wrapper = getWrapper();
      expect(wrapper.attributes()['type']).toEqual('button');
    });
    test('has the provided button type', () => {
      const wrapper = getWrapper({ props: { type: 'submit' } });
      expect(wrapper.attributes()['type']).toEqual('submit');
    });
    test('defaults to the data-ui button', () => {
      const wrapper = getWrapper();
      expect(wrapper.attributes()['data-ui']).toEqual('button');
    });
    test('has the provided data-ui', () => {
      const wrapper = getWrapper({ props: { dataUi: 'my-data-ui' } });
      expect(wrapper.attributes()['data-ui']).toEqual('my-data-ui');
    });
    test('has the provided class', () => {
      const wrapper = getWrapper({ props: { class: 'my-class' } });
      expect(wrapper.classes()).toContain('my-class');
    });
  });
  describe('functionality', () => {
    test('click event handler fires when button is clicked', async () => {
      const onClick = vi.fn();
      const wrapper = getWrapper({ attrs: { onClick } });
      await wrapper.trigger('click');
      expect(onClick).toHaveBeenCalled();
    });
  });
});
