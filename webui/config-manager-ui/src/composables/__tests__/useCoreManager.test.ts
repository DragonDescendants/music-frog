import { describe, expect, it, vi, beforeEach } from 'vitest';
import { ref } from 'vue';
import { useCoreManager } from '../useCoreManager';
import { api } from '../../api';

vi.mock('../../api', () => ({
  api: {
    listCoreVersions: vi.fn(),
    getLatestStableCore: vi.fn(),
    activateCoreVersion: vi.fn(),
    downloadCoreVersion: vi.fn(),
    updateStableCore: vi.fn(),
  },
}));

vi.mock('vue-i18n', () => ({
  useI18n: () => ({
    t: (key: string) => key,
  }),
}));

function buildBusy() {
  return {
    busy: ref(false),
    startBusy: vi.fn(),
    updateBusyDetail: vi.fn(),
    endBusy: vi.fn(),
  };
}

describe('useCoreManager', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('refreshes core versions with latest stable info', async () => {
    vi.mocked(api.listCoreVersions).mockResolvedValue({
      current: 'v1.20.0',
      versions: ['v1.20.0', 'v1.19.0'],
    });
    vi.mocked(api.getLatestStableCore).mockResolvedValue({
      version: 'v1.21.0',
      release_date: '2026-02-01T00:00:00Z',
    });
    const setStatus = vi.fn();
    const pushToast = vi.fn();

    const manager = useCoreManager(setStatus, pushToast, vi.fn(), buildBusy());
    await manager.refreshCoreVersions();

    expect(manager.coreCurrent.value).toBe('v1.20.0');
    expect(manager.coreVersions.value).toEqual(['v1.20.0', 'v1.19.0']);
    expect(manager.coreLatestStable.value).toBe('v1.21.0');
    expect(manager.coreLatestStableDate.value).toBe('2026-02-01T00:00:00Z');
    expect(setStatus).toHaveBeenCalledWith('app.core_refreshed', 'app.core_current');
  });

  it('activates core version and waits for rebuild', async () => {
    vi.mocked(api.activateCoreVersion).mockResolvedValue(undefined);
    vi.mocked(api.listCoreVersions).mockResolvedValue({ current: 'v1.20.0', versions: ['v1.20.0'] });
    vi.mocked(api.getLatestStableCore).mockResolvedValue({
      version: 'v1.21.0',
      release_date: '2026-02-01T00:00:00Z',
    });
    const setStatus = vi.fn();
    const pushToast = vi.fn();
    const waitForRebuild = vi.fn().mockResolvedValue(undefined);
    const busy = buildBusy();

    const manager = useCoreManager(setStatus, pushToast, waitForRebuild, busy);
    await manager.activateCore('  v1.20.0  ');

    expect(api.activateCoreVersion).toHaveBeenCalledWith('v1.20.0');
    expect(waitForRebuild).toHaveBeenCalledWith('app.switch_core_busy');
    expect(busy.startBusy).toHaveBeenCalled();
    expect(busy.endBusy).toHaveBeenCalled();
    expect(manager.coreOperationText.value).toBe('core.operation_switched');
  });

  it('downloads core version and handles already installed result', async () => {
    vi.mocked(api.downloadCoreVersion).mockResolvedValue({
      version: 'v1.20.0',
      downloaded: false,
      already_installed: true,
    });
    vi.mocked(api.listCoreVersions).mockResolvedValue({ current: null, versions: ['v1.20.0'] });
    vi.mocked(api.getLatestStableCore).mockResolvedValue({
      version: 'v1.20.0',
      release_date: '2026-02-01T00:00:00Z',
    });
    const setStatus = vi.fn();
    const pushToast = vi.fn();

    const manager = useCoreManager(setStatus, pushToast, vi.fn(), buildBusy());
    await manager.downloadCoreVersion('v1.20.0');

    expect(api.downloadCoreVersion).toHaveBeenCalledWith('v1.20.0');
    expect(setStatus).toHaveBeenCalledWith('app.download_core_skipped', 'v1.20.0');
    expect(manager.coreOperationText.value).toBe('core.operation_already');
  });

  it('updates stable core and waits for rebuild when scheduled', async () => {
    vi.mocked(api.updateStableCore).mockResolvedValue({
      version: 'v1.21.0',
      downloaded: true,
      already_installed: false,
      rebuild_scheduled: true,
    });
    vi.mocked(api.listCoreVersions).mockResolvedValue({ current: 'v1.21.0', versions: ['v1.21.0'] });
    vi.mocked(api.getLatestStableCore).mockResolvedValue({
      version: 'v1.21.0',
      release_date: '2026-02-01T00:00:00Z',
    });
    const setStatus = vi.fn();
    const pushToast = vi.fn();
    const waitForRebuild = vi.fn().mockResolvedValue(undefined);

    const manager = useCoreManager(setStatus, pushToast, waitForRebuild, buildBusy());
    await manager.updateStableCore();

    expect(api.updateStableCore).toHaveBeenCalled();
    expect(waitForRebuild).toHaveBeenCalledWith('app.update_stable_busy');
    expect(setStatus).toHaveBeenCalledWith('app.update_stable_success', 'v1.21.0');
    expect(manager.coreOperationText.value).toBe('core.operation_updated_stable');
  });

  it('rejects empty version before download', async () => {
    const setStatus = vi.fn();
    const pushToast = vi.fn();

    const manager = useCoreManager(setStatus, pushToast, vi.fn(), buildBusy());
    await manager.downloadCoreVersion('   ');

    expect(api.downloadCoreVersion).not.toHaveBeenCalled();
    expect(setStatus).toHaveBeenCalledWith('app.core_version_empty');
  });
});
