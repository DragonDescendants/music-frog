import { describe, expect, it, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import CorePanel from '../CorePanel.vue';
import { createTestI18n } from '../../test-utils/i18n';

const i18n = createTestI18n();

describe('CorePanel', () => {
  it('renders current version correctly', () => {
    const wrapper = mount(CorePanel, {
      global: { plugins: [i18n] },
      props: {
        coreVersions: ['v1.18.0', 'v1.19.0'],
        coreCurrent: 'v1.19.0',
        coreLatestStable: 'v1.20.0',
        coreLatestStableDate: null,
        coreOperationText: '',
      },
    });

    expect(wrapper.text()).toContain('Current: v1.19.0');
    expect(wrapper.findAll('li').length).toBe(2);
  });

  it('shows empty message when no versions', () => {
    const wrapper = mount(CorePanel, {
      global: { plugins: [i18n] },
      props: {
        coreVersions: [],
        coreCurrent: null,
        coreLatestStable: null,
        coreLatestStableDate: null,
        coreOperationText: '',
      },
    });

    expect(wrapper.text()).toContain('Default Core');
    expect(wrapper.text()).toContain('No versions downloaded');
  });

  it('emits activate event when button clicked', async () => {
    const wrapper = mount(CorePanel, {
      global: { plugins: [i18n] },
      props: {
        coreVersions: ['v1.18.0'],
        coreCurrent: 'v1.19.0',
        coreLatestStable: 'v1.20.0',
        coreLatestStableDate: null,
        coreOperationText: '',
      },
    });

    const useBtn = wrapper.findAll('button').find((button) => button.text() === 'Use');
    expect(useBtn).toBeDefined();
    await useBtn?.trigger('click');

    expect(wrapper.emitted('activate')).toBeTruthy();
    expect(wrapper.emitted('activate')![0]).toEqual(['v1.18.0']);
  });

  it('emits download and update-stable events', async () => {
    const wrapper = mount(CorePanel, {
      global: { plugins: [i18n] },
      props: {
        coreVersions: ['v1.18.0'],
        coreCurrent: 'v1.18.0',
        coreLatestStable: 'v1.20.0',
        coreLatestStableDate: '2026-01-01T00:00:00Z',
        coreOperationText: 'idle',
      },
    });

    const downloadBtn = wrapper.findAll('button').find((button) => button.text() === 'Download Stable');
    expect(downloadBtn).toBeDefined();
    await downloadBtn?.trigger('click');
    expect(wrapper.emitted('download')?.[0]).toEqual(['v1.20.0']);

    const updateBtn = wrapper.findAll('button').find((button) => button.text() === 'Update To Stable');
    expect(updateBtn).toBeDefined();
    await updateBtn?.trigger('click');
    expect(wrapper.emitted('update-stable')).toBeTruthy();
  });
});
