import { computed, onBeforeUnmount, ref, watch, type Ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { api, runtimeLogsUrl } from '../api';
import type {
  RuntimeConnection,
  RuntimeIpCheckResponse,
  RuntimeLogLevel,
  RuntimeMemoryData,
  RuntimeProxyDelayNode,
  RuntimeTrafficSnapshot,
} from '../types';
import type { ToastTone } from './useToasts';

type RuntimeManagerOptions = {
  activeSection: Ref<string>;
  pushToast: (message: string, tone?: ToastTone) => void;
};

const POLL_INTERVAL_MS = 2000;
const MAX_LOG_LINES = 500;
const DEFAULT_RUNTIME_DELAY_TEST_URL = 'http://www.gstatic.com/generate_204';
const DEFAULT_RUNTIME_DELAY_TIMEOUT_MS = 5000;

type DelaySortKey = 'delay_asc' | 'delay_desc' | 'name_asc' | 'name_desc';

export function useRuntimeManager(options: RuntimeManagerOptions) {
  const { t } = useI18n();

  const connectionFilter = ref('');
  const connections = ref<RuntimeConnection[]>([]);
  const uploadTotal = ref(0);
  const downloadTotal = ref(0);

  const traffic = ref<RuntimeTrafficSnapshot | null>(null);
  const memory = ref<RuntimeMemoryData | null>(null);
  const ipInfo = ref<RuntimeIpCheckResponse | null>(null);

  const logs = ref<string[]>([]);
  const logLevel = ref<RuntimeLogLevel>('info');
  const streamConnected = ref(false);
  const autoRefresh = ref(true);
  const delaySort = ref<DelaySortKey>('delay_asc');
  const delayNodes = ref<RuntimeProxyDelayNode[]>([]);
  const delayTestUrl = ref(DEFAULT_RUNTIME_DELAY_TEST_URL);
  const delayTimeoutMs = ref(DEFAULT_RUNTIME_DELAY_TIMEOUT_MS);

  const loadingConnections = ref(false);
  const loadingOverview = ref(false);
  const loadingDelayNodes = ref(false);
  const closingConnectionId = ref('');
  const closingAll = ref(false);
  const testingDelayProxy = ref('');
  const testingAllDelays = ref(false);

  let pollTimer: number | null = null;
  let pollTick = 0;
  let logSource: EventSource | null = null;

  const filteredConnections = computed(() => {
    const keyword = connectionFilter.value.trim().toLowerCase();
    if (!keyword) {
      return connections.value;
    }
    return connections.value.filter((connection) => {
      const metadata = connection.metadata || {};
      return (
        connection.id?.toLowerCase().includes(keyword) ||
        metadata.host?.toLowerCase().includes(keyword) ||
        metadata.processPath?.toLowerCase().includes(keyword) ||
        metadata.sourceIP?.toLowerCase().includes(keyword) ||
        metadata.destinationIP?.toLowerCase().includes(keyword) ||
        connection.rule?.toLowerCase().includes(keyword)
      );
    });
  });

  const sortedDelayNodes = computed(() => {
    const nodes = [...delayNodes.value];
    nodes.sort((left, right) => {
      if (delaySort.value === 'name_asc') {
        return left.name.localeCompare(right.name);
      }
      if (delaySort.value === 'name_desc') {
        return right.name.localeCompare(left.name);
      }
      if (delaySort.value === 'delay_desc') {
        return compareDelayNodes(left, right, 'desc');
      }
      return compareDelayNodes(left, right, 'asc');
    });
    return nodes;
  });

  function appendLog(message: string) {
    logs.value.push(message);
    if (logs.value.length > MAX_LOG_LINES) {
      logs.value.splice(0, logs.value.length - MAX_LOG_LINES);
    }
  }

  function clearLogs() {
    logs.value = [];
  }

  function closeLogStream() {
    if (logSource) {
      logSource.close();
      logSource = null;
    }
    streamConnected.value = false;
  }

  function openLogStream() {
    closeLogStream();
    if (options.activeSection.value !== 'runtime') {
      return;
    }
    const source = new EventSource(runtimeLogsUrl(logLevel.value));
    source.onopen = () => {
      streamConnected.value = true;
    };
    source.onerror = () => {
      streamConnected.value = false;
    };
    source.onmessage = (event) => {
      try {
        const payload = JSON.parse(event.data) as { message?: string };
        if (payload && typeof payload.message === 'string') {
          appendLog(payload.message);
          return;
        }
      } catch {
        // keep raw payload when parsing fails
      }
      appendLog(String(event.data || ''));
    };
    logSource = source;
  }

  function stopPolling() {
    if (pollTimer !== null) {
      window.clearInterval(pollTimer);
      pollTimer = null;
    }
  }

  async function refreshConnections(silent = false) {
    loadingConnections.value = true;
    try {
      const data = await api.listRuntimeConnections();
      connections.value = data.connections || [];
      uploadTotal.value = data.uploadTotal || 0;
      downloadTotal.value = data.downloadTotal || 0;
    } catch (err) {
      if (!silent) {
        options.pushToast((err as Error).message || String(err), 'error');
      }
    } finally {
      loadingConnections.value = false;
    }
  }

  async function refreshTraffic(silent = false) {
    try {
      traffic.value = await api.getRuntimeTraffic();
    } catch (err) {
      if (!silent) {
        options.pushToast((err as Error).message || String(err), 'error');
      }
    }
  }

  async function refreshMemory(silent = false) {
    try {
      memory.value = await api.getRuntimeMemory();
    } catch (err) {
      if (!silent) {
        options.pushToast((err as Error).message || String(err), 'error');
      }
    }
  }

  async function refreshIp(silent = false) {
    try {
      ipInfo.value = await api.getRuntimeIp();
    } catch (err) {
      if (!silent) {
        options.pushToast((err as Error).message || String(err), 'error');
      }
    }
  }

  async function refreshOverview(silent = false) {
    loadingOverview.value = true;
    try {
      await Promise.all([
        refreshTraffic(silent),
        refreshMemory(silent),
        refreshIp(silent),
      ]);
    } finally {
      loadingOverview.value = false;
    }
  }

  async function refreshProxyDelays(silent = false) {
    loadingDelayNodes.value = true;
    try {
      const data = await api.listRuntimeProxyDelayNodes();
      delayNodes.value = Array.isArray(data.nodes) ? data.nodes : [];
      if (typeof data.default_test_url === 'string' && data.default_test_url.trim()) {
        delayTestUrl.value = data.default_test_url.trim();
      }
      if (
        Number.isFinite(data.default_timeout_ms) &&
        Number(data.default_timeout_ms) > 0
      ) {
        delayTimeoutMs.value = Number(data.default_timeout_ms);
      }
    } catch (err) {
      if (!silent) {
        options.pushToast((err as Error).message || String(err), 'error');
      }
    } finally {
      loadingDelayNodes.value = false;
    }
  }

  async function refreshRuntimeData(silent = false) {
    await Promise.all([
      refreshConnections(silent),
      refreshOverview(silent),
      refreshProxyDelays(silent),
    ]);
  }

  async function pollRuntimeData() {
    pollTick += 1;
    await Promise.all([refreshTraffic(true), refreshMemory(true)]);
    if (pollTick % 2 === 0) {
      await refreshConnections(true);
    }
  }

  function startPolling() {
    stopPolling();
    pollTimer = window.setInterval(() => {
      void pollRuntimeData();
    }, POLL_INTERVAL_MS);
  }

  async function closeConnection(id: string) {
    const connectionId = id.trim();
    if (!connectionId || closingConnectionId.value) {
      return;
    }
    closingConnectionId.value = connectionId;
    try {
      await api.closeRuntimeConnection(connectionId);
      await Promise.all([refreshConnections(true), refreshTraffic(true)]);
      options.pushToast(t('runtime.close_connection_success'));
    } catch (err) {
      options.pushToast((err as Error).message || String(err), 'error');
    } finally {
      closingConnectionId.value = '';
    }
  }

  async function closeAllConnections() {
    if (closingAll.value) {
      return;
    }
    const confirmed = window.confirm(t('runtime.close_all_confirm'));
    if (!confirmed) {
      return;
    }
    closingAll.value = true;
    try {
      await api.closeAllRuntimeConnections();
      await Promise.all([refreshConnections(true), refreshTraffic(true)]);
      options.pushToast(t('runtime.close_all_success'));
    } catch (err) {
      options.pushToast((err as Error).message || String(err), 'error');
    } finally {
      closingAll.value = false;
    }
  }

  async function testProxyDelay(proxy: string) {
    const name = proxy.trim();
    if (!name || testingDelayProxy.value || testingAllDelays.value) {
      return;
    }
    testingDelayProxy.value = name;
    try {
      const response = await api.testRuntimeProxyDelay({
        proxy: name,
        test_url: delayTestUrl.value,
        timeout_ms: delayTimeoutMs.value,
      });
      upsertDelayNode({
        name: response.proxy,
        proxy_type: findDelayNode(response.proxy)?.proxy_type || 'Unknown',
        delay_ms: response.delay_ms,
        tested_at: response.tested_at,
      });
      options.pushToast(
        t('runtime.delay_test_success', { proxy: response.proxy, delay: response.delay_ms }),
      );
    } catch (err) {
      options.pushToast((err as Error).message || String(err), 'error');
    } finally {
      testingDelayProxy.value = '';
    }
  }

  async function testAllProxyDelays() {
    if (testingAllDelays.value || testingDelayProxy.value) {
      return;
    }
    testingAllDelays.value = true;
    try {
      const response = await api.testAllRuntimeProxyDelays({
        test_url: delayTestUrl.value,
        timeout_ms: delayTimeoutMs.value,
      });
      await refreshProxyDelays(true);
      options.pushToast(
        t('runtime.delay_test_all_success', {
          success: response.success_count,
          failed: response.failed_count,
        }),
      );
    } catch (err) {
      options.pushToast((err as Error).message || String(err), 'error');
    } finally {
      testingAllDelays.value = false;
    }
  }

  function findDelayNode(proxy: string) {
    return delayNodes.value.find((node) => node.name === proxy);
  }

  function upsertDelayNode(node: RuntimeProxyDelayNode) {
    const index = delayNodes.value.findIndex((item) => item.name === node.name);
    if (index >= 0) {
      delayNodes.value[index] = {
        ...delayNodes.value[index],
        ...node,
      };
      return;
    }
    delayNodes.value.push(node);
  }

  watch(logLevel, () => {
    if (options.activeSection.value === 'runtime') {
      openLogStream();
    }
  });

  watch(
    () => [options.activeSection.value, autoRefresh.value],
    ([section, enabled]) => {
      if (section !== 'runtime') {
        stopPolling();
        closeLogStream();
        return;
      }
      void refreshRuntimeData(true);
      openLogStream();
      if (enabled) {
        startPolling();
      } else {
        stopPolling();
      }
    },
    { immediate: true },
  );

  onBeforeUnmount(() => {
    stopPolling();
    closeLogStream();
  });

  return {
    autoRefresh,
    delayNodes,
    delaySort,
    delayTestUrl,
    delayTimeoutMs,
    loadingDelayNodes,
    testingDelayProxy,
    testingAllDelays,
    closingAll,
    closingConnectionId,
    connectionFilter,
    connections,
    downloadTotal,
    filteredConnections,
    sortedDelayNodes,
    ipInfo,
    loadingConnections,
    loadingOverview,
    logLevel,
    logs,
    memory,
    refreshConnections,
    refreshIp,
    refreshOverview,
    refreshProxyDelays,
    refreshRuntimeData,
    clearLogs,
    closeAllConnections,
    closeConnection,
    testAllProxyDelays,
    testProxyDelay,
    streamConnected,
    traffic,
    uploadTotal,
  };
}

function compareDelayNodes(
  left: RuntimeProxyDelayNode,
  right: RuntimeProxyDelayNode,
  direction: 'asc' | 'desc',
) {
  const leftDelay = asSortableDelay(left.delay_ms);
  const rightDelay = asSortableDelay(right.delay_ms);
  if (leftDelay === null && rightDelay === null) {
    return left.name.localeCompare(right.name);
  }
  if (leftDelay === null) {
    return 1;
  }
  if (rightDelay === null) {
    return -1;
  }
  const diff = direction === 'asc' ? leftDelay - rightDelay : rightDelay - leftDelay;
  if (diff !== 0) {
    return diff;
  }
  return left.name.localeCompare(right.name);
}

function asSortableDelay(value?: number | null): number | null {
  if (value === null || value === undefined) {
    return null;
  }
  const num = Number(value);
  if (!Number.isFinite(num) || num < 0) {
    return null;
  }
  return num;
}
