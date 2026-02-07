import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { api } from '../api';
import type { ToastTone } from './useToasts';

type BusyActions = {
  busy: { value: boolean };
  startBusy: (message: string, detail: string) => void;
  updateBusyDetail: (detail: string) => void;
  endBusy: () => void;
};

export function useCoreManager(
  setStatus: (message: string, detail?: string) => void,
  pushToast: (message: string, tone?: ToastTone) => void,
  waitForRebuild: (label: string) => Promise<void>,
  busy: BusyActions,
) {
  const { t } = useI18n();
  const coreVersions = ref<string[]>([]);
  const coreCurrent = ref<string | null>(null);
  const coreLatestStable = ref<string | null>(null);
  const coreLatestStableDate = ref<string | null>(null);
  const coreOperationText = ref('');

  async function refreshCoreVersions(silent = false) {
    try {
      const [versionsData, stableData] = await Promise.all([
        api.listCoreVersions(),
        api.getLatestStableCore().catch(() => null),
      ]);
      coreVersions.value = versionsData.versions;
      coreCurrent.value = versionsData.current || null;
      if (stableData) {
        coreLatestStable.value = stableData.version || null;
        coreLatestStableDate.value = stableData.release_date || null;
      }
      if (!silent) {
        setStatus(
          t('app.core_refreshed'),
          versionsData.current
            ? t('app.core_current', { version: versionsData.current })
            : t('app.core_default'),
        );
      }
    } catch (err) {
      const message = (err as Error).message || String(err);
      if (!silent) {
        setStatus(t('app.load_core_failed'), message);
      }
      pushToast(message, 'error');
    }
  }

  async function refreshLatestStable(silent = false) {
    try {
      const data = await api.getLatestStableCore();
      coreLatestStable.value = data.version || null;
      coreLatestStableDate.value = data.release_date || null;
      if (!silent) {
        setStatus(t('app.core_latest_refreshed'), data.version);
      }
    } catch (err) {
      const message = (err as Error).message || String(err);
      if (!silent) {
        setStatus(t('app.load_core_latest_failed'), message);
      }
      pushToast(message, 'error');
    }
  }

  async function activateCore(version: string) {
    if (busy.busy.value) {
      return;
    }
    const trimmed = version.trim();
    if (!trimmed) {
      setStatus(t('app.core_version_empty'));
      return;
    }
    busy.startBusy(t('app.switch_core_busy'), t('app.switch_core_detail', { version }));
    coreOperationText.value = t('core.operation_switching', { version: trimmed });
    try {
      await api.activateCoreVersion(trimmed);
      setStatus(t('app.switch_core_status'), t('app.switch_core_detail', { version: trimmed }));
      busy.updateBusyDetail(t('app.switch_rebuild'));
      await waitForRebuild(t('app.switch_core_busy'));
      setStatus(t('app.switch_core_success'), trimmed);
      coreOperationText.value = t('core.operation_switched', { version: trimmed });
    } catch (err) {
      const message = (err as Error).message || String(err);
      setStatus(t('app.switch_core_failed'), message);
      pushToast(message, 'error');
      coreOperationText.value = t('core.operation_failed', { message });
    } finally {
      await refreshCoreVersions(true);
      busy.endBusy();
    }
  }

  async function downloadCoreVersion(version: string) {
    if (busy.busy.value) {
      return;
    }
    const trimmed = version.trim();
    if (!trimmed) {
      setStatus(t('app.core_version_empty'));
      return;
    }
    busy.startBusy(t('app.download_core_busy'), t('app.download_core_detail', { version: trimmed }));
    coreOperationText.value = t('core.operation_downloading', { version: trimmed });
    try {
      const result = await api.downloadCoreVersion(trimmed);
      if (result.downloaded) {
        setStatus(t('app.download_core_success'), trimmed);
        coreOperationText.value = t('core.operation_downloaded', { version: trimmed });
      } else if (result.already_installed) {
        setStatus(t('app.download_core_skipped'), trimmed);
        coreOperationText.value = t('core.operation_already', { version: trimmed });
      } else {
        setStatus(t('app.download_core_success'), trimmed);
        coreOperationText.value = t('core.operation_downloaded', { version: trimmed });
      }
    } catch (err) {
      const message = (err as Error).message || String(err);
      setStatus(t('app.download_core_failed'), message);
      pushToast(message, 'error');
      coreOperationText.value = t('core.operation_failed', { message });
    } finally {
      await refreshCoreVersions(true);
      busy.endBusy();
    }
  }

  async function updateStableCore() {
    if (busy.busy.value) {
      return;
    }
    busy.startBusy(t('app.update_stable_busy'), t('app.update_stable_checking'));
    coreOperationText.value = t('core.operation_checking_stable');
    try {
      busy.updateBusyDetail(t('app.update_stable_downloading'));
      coreOperationText.value = t('core.operation_updating_stable');
      const result = await api.updateStableCore();
      setStatus(t('app.update_stable_status'), result.version);
      if (result.rebuild_scheduled) {
        busy.updateBusyDetail(t('app.switch_rebuild'));
        await waitForRebuild(t('app.update_stable_busy'));
      }
      setStatus(t('app.update_stable_success'), result.version);
      coreOperationText.value = t('core.operation_updated_stable', { version: result.version });
    } catch (err) {
      const message = (err as Error).message || String(err);
      setStatus(t('app.update_stable_failed'), message);
      pushToast(message, 'error');
      coreOperationText.value = t('core.operation_failed', { message });
    } finally {
      await refreshCoreVersions(true);
      busy.endBusy();
    }
  }

  return {
    coreVersions,
    coreCurrent,
    coreLatestStable,
    coreLatestStableDate,
    coreOperationText,
    refreshCoreVersions,
    refreshLatestStable,
    activateCore,
    downloadCoreVersion,
    updateStableCore,
  };
}
