import { reactive, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { api } from '../api';
import type { DnsConfig, FakeIpConfig, RuleEntry, RuleProvider, TunConfig } from '../types';
import type { ToastTone } from './useToasts';
import { useRebuildWatcher } from './useRebuildWatcher';

type BusyActions = {
  busy: { value: boolean };
  startBusy: (message: string, detail: string) => void;
  updateBusyDetail: (detail: string) => void;
  endBusy: () => void;
};

export function useAdvancedSettings(pushToast: (message: string, tone?: ToastTone) => void, busy: BusyActions) {
  const { t } = useI18n();
  const { waitForRebuild } = useRebuildWatcher(busy.updateBusyDetail);

  const dnsConfig = ref<DnsConfig>({});
  const fakeIpConfig = ref<FakeIpConfig>({});
  const tunConfig = ref<TunConfig>({});
  const rules = ref<RuleEntry[]>([]);
  const ruleProvidersJson = ref('{}');
  const proxyProvidersJson = ref('{}');
  const snifferJson = ref('{}');
  const dirty = reactive({
    dns: false,
    fakeIp: false,
    tun: false,
    rules: false,
    ruleProviders: false,
    proxyProviders: false,
    sniffer: false,
  });
  const suppressDirty = {
    dns: false,
    fakeIp: false,
    tun: false,
    rules: false,
    ruleProviders: false,
    proxyProviders: false,
    sniffer: false,
  };

  function setCleanValue<K extends keyof typeof dirty, T>(key: K, target: { value: T }, value: T) {
    suppressDirty[key] = true;
    target.value = value;
    dirty[key] = false;
    suppressDirty[key] = false;
  }

  watch(
    dnsConfig,
    () => {
      if (!suppressDirty.dns) {
        dirty.dns = true;
      }
    },
    { deep: true, flush: 'sync' },
  );
  watch(
    fakeIpConfig,
    () => {
      if (!suppressDirty.fakeIp) {
        dirty.fakeIp = true;
      }
    },
    { deep: true, flush: 'sync' },
  );
  watch(
    tunConfig,
    () => {
      if (!suppressDirty.tun) {
        dirty.tun = true;
      }
    },
    { deep: true, flush: 'sync' },
  );
  watch(
    rules,
    () => {
      if (!suppressDirty.rules) {
        dirty.rules = true;
      }
    },
    { deep: true, flush: 'sync' },
  );
  watch(
    ruleProvidersJson,
    () => {
      if (!suppressDirty.ruleProviders) {
        dirty.ruleProviders = true;
      }
    },
    { flush: 'sync' },
  );
  watch(
    proxyProvidersJson,
    () => {
      if (!suppressDirty.proxyProviders) {
        dirty.proxyProviders = true;
      }
    },
    { flush: 'sync' },
  );
  watch(
    snifferJson,
    () => {
      if (!suppressDirty.sniffer) {
        dirty.sniffer = true;
      }
    },
    { flush: 'sync' },
  );

  function isPlainObject(value: unknown): value is Record<string, unknown> {
    return typeof value === 'object' && value !== null && !Array.isArray(value);
  }

  function isRuleProvider(value: Record<string, unknown>): value is RuleProvider {
    if (typeof value.type !== 'string' || value.type.trim().length === 0) {
      return false;
    }
    if (value.behavior !== undefined && typeof value.behavior !== 'string') {
      return false;
    }
    if (value.path !== undefined && typeof value.path !== 'string') {
      return false;
    }
    if (value.url !== undefined && typeof value.url !== 'string') {
      return false;
    }
    if (value.interval !== undefined && (typeof value.interval !== 'number' || !Number.isFinite(value.interval))) {
      return false;
    }
    if (value.format !== undefined && typeof value.format !== 'string') {
      return false;
    }
    return true;
  }

  function areRuleProviders(value: Record<string, unknown>): value is Record<string, RuleProvider> {
    return Object.values(value).every((entry) => isPlainObject(entry) && isRuleProvider(entry));
  }

  function isProxyProvider(value: Record<string, unknown>): boolean {
    return typeof value.type === 'string' && value.type.trim().length > 0;
  }

  function areProxyProviders(value: Record<string, unknown>): value is Record<string, Record<string, unknown>> {
    return Object.values(value).every((entry) => isPlainObject(entry) && isProxyProvider(entry));
  }

  function parseJsonObject(raw: string, errorMessage: string): Record<string, unknown> {
    const trimmed = raw.trim();
    const parsed: unknown = trimmed ? JSON.parse(trimmed) : {};
    if (!isPlainObject(parsed)) {
      throw new Error(errorMessage);
    }
    return parsed;
  }

  async function refreshDnsConfig(silent = false) {
    try {
      const data = await api.getDnsConfig();
      setCleanValue('dns', dnsConfig, data || {});
    } catch (err) {
      const message = (err as Error).message || String(err);
      if (!silent) {
        pushToast(message, 'error');
      }
    }
  }

  async function refreshFakeIpConfig(silent = false) {
    try {
      const data = await api.getFakeIpConfig();
      setCleanValue('fakeIp', fakeIpConfig, data || {});
    } catch (err) {
      const message = (err as Error).message || String(err);
      if (!silent) {
        pushToast(message, 'error');
      }
    }
  }

  async function refreshRuleProviders(silent = false) {
    try {
      const data = await api.getRuleProviders();
      setCleanValue(
        'ruleProviders',
        ruleProvidersJson,
        JSON.stringify(data.providers || {}, null, 2),
      );
    } catch (err) {
      const message = (err as Error).message || String(err);
      if (!silent) {
        pushToast(message, 'error');
      }
    }
  }

  async function refreshProxyProviders(silent = false) {
    try {
      const data = await api.getProxyProviders();
      setCleanValue(
        'proxyProviders',
        proxyProvidersJson,
        JSON.stringify(data.providers || {}, null, 2),
      );
    } catch (err) {
      const message = (err as Error).message || String(err);
      if (!silent) {
        pushToast(message, 'error');
      }
    }
  }

  async function refreshSnifferConfig(silent = false) {
    try {
      const data = await api.getSnifferConfig();
      setCleanValue(
        'sniffer',
        snifferJson,
        JSON.stringify(data || {}, null, 2),
      );
    } catch (err) {
      const message = (err as Error).message || String(err);
      if (!silent) {
        pushToast(message, 'error');
      }
    }
  }

  async function refreshRules(silent = false) {
    try {
      const data = await api.getRules();
      setCleanValue('rules', rules, data.rules || []);
    } catch (err) {
      const message = (err as Error).message || String(err);
      if (!silent) {
        pushToast(message, 'error');
      }
    }
  }

  async function refreshRulesAndProviders(silent = false) {
    await Promise.all([
      refreshRules(silent),
      refreshRuleProviders(silent),
      refreshProxyProviders(silent),
      refreshSnifferConfig(silent),
    ]);
  }

  async function refreshTunConfig(silent = false) {
    try {
      const data = await api.getTunConfig();
      setCleanValue('tun', tunConfig, data || {});
    } catch (err) {
      const message = (err as Error).message || String(err);
      if (!silent) {
        pushToast(message, 'error');
      }
    }
  }

  async function saveDnsConfig() {
    if (busy.busy.value) {
      return;
    }
    busy.startBusy(t('dns.saving'), t('dns.saving_detail'));
    try {
      await api.saveDnsConfig(dnsConfig.value);
      await waitForRebuild(t('dns.saving'));
      pushToast(t('dns.save_success'));
    } catch (err) {
      const message = (err as Error).message || String(err);
      pushToast(message, 'error');
    } finally {
      await refreshDnsConfig(true);
      busy.endBusy();
    }
  }

  async function saveFakeIpConfig() {
    if (busy.busy.value) {
      return;
    }
    busy.startBusy(t('fake_ip.saving'), t('fake_ip.saving_detail'));
    try {
      await api.saveFakeIpConfig(fakeIpConfig.value);
      await waitForRebuild(t('fake_ip.saving'));
      pushToast(t('fake_ip.save_success'));
    } catch (err) {
      const message = (err as Error).message || String(err);
      pushToast(message, 'error');
    } finally {
      await refreshFakeIpConfig(true);
      busy.endBusy();
    }
  }

  async function flushFakeIpCache() {
    if (busy.busy.value) {
      return;
    }
    busy.startBusy(t('fake_ip.flushing'), t('fake_ip.flushing_detail'));
    try {
      const result = await api.flushFakeIpCache();
      if (result.removed) {
        pushToast(t('fake_ip.flush_done'));
      } else {
        pushToast(t('fake_ip.flush_none'));
      }
    } catch (err) {
      const message = (err as Error).message || String(err);
      pushToast(message, 'error');
    } finally {
      busy.endBusy();
    }
  }

  async function saveRuleProviders() {
    if (busy.busy.value) {
      return;
    }
    busy.startBusy(t('rules.saving_providers'), t('rules.saving_detail'));
    try {
      const parsed = parseJsonObject(ruleProvidersJson.value, t('rules.invalid_providers'));
      if (!areRuleProviders(parsed)) {
        throw new Error(t('rules.invalid_provider_items'));
      }
      await api.saveRuleProviders({ providers: parsed });
      await waitForRebuild(t('rules.saving_providers'));
      pushToast(t('rules.save_providers_success'));
    } catch (err) {
      const message = (err as Error).message || String(err);
      pushToast(message, 'error');
    } finally {
      await refreshRuleProviders(true);
      busy.endBusy();
    }
  }

  async function saveProxyProviders() {
    if (busy.busy.value) {
      return;
    }
    busy.startBusy(t('rules.saving_proxy_providers'), t('rules.saving_detail'));
    try {
      const parsed = parseJsonObject(proxyProvidersJson.value, t('rules.invalid_proxy_providers'));
      if (!areProxyProviders(parsed)) {
        throw new Error(t('rules.invalid_proxy_provider_items'));
      }
      await api.saveProxyProviders({ providers: parsed });
      await waitForRebuild(t('rules.saving_proxy_providers'));
      pushToast(t('rules.save_proxy_providers_success'));
    } catch (err) {
      const message = (err as Error).message || String(err);
      pushToast(message, 'error');
    } finally {
      await refreshProxyProviders(true);
      busy.endBusy();
    }
  }

  async function saveSnifferConfig() {
    if (busy.busy.value) {
      return;
    }
    busy.startBusy(t('rules.saving_sniffer'), t('rules.saving_detail'));
    try {
      const parsed = parseJsonObject(snifferJson.value, t('rules.invalid_sniffer'));
      await api.saveSnifferConfig(parsed);
      await waitForRebuild(t('rules.saving_sniffer'));
      pushToast(t('rules.save_sniffer_success'));
    } catch (err) {
      const message = (err as Error).message || String(err);
      pushToast(message, 'error');
    } finally {
      await refreshSnifferConfig(true);
      busy.endBusy();
    }
  }

  async function saveRules() {
    if (busy.busy.value) {
      return;
    }
    busy.startBusy(t('rules.saving_rules'), t('rules.saving_detail'));
    try {
      await api.saveRules({ rules: rules.value });
      await waitForRebuild(t('rules.saving_rules'));
      pushToast(t('rules.save_rules_success'));
    } catch (err) {
      const message = (err as Error).message || String(err);
      pushToast(message, 'error');
    } finally {
      await refreshRules(true);
      busy.endBusy();
    }
  }

  async function saveTunConfig() {
    if (busy.busy.value) {
      return;
    }
    busy.startBusy(t('tun.saving'), t('tun.saving_detail'));
    try {
      await api.saveTunConfig(tunConfig.value);
      await waitForRebuild(t('tun.saving'));
      pushToast(t('tun.save_success'));
    } catch (err) {
      const message = (err as Error).message || String(err);
      pushToast(message, 'error');
    } finally {
      await refreshTunConfig(true);
      busy.endBusy();
    }
  }

  return {
    dnsConfig,
    fakeIpConfig,
    tunConfig,
    rules,
    ruleProvidersJson,
    proxyProvidersJson,
    snifferJson,
    dirty,
    refreshDnsConfig,
    refreshFakeIpConfig,
    refreshRuleProviders,
    refreshProxyProviders,
    refreshRules,
    refreshRulesAndProviders,
    refreshSnifferConfig,
    refreshTunConfig,
    saveDnsConfig,
    saveFakeIpConfig,
    flushFakeIpCache,
    saveRuleProviders,
    saveProxyProviders,
    saveSnifferConfig,
    saveRules,
    saveTunConfig,
  };
}
