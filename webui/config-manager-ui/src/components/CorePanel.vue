<template>
  <PanelCard>
    <PanelHeader>
      <template #title>
        <PanelTitle :text="$t('core.title')" />
      </template>
      <template #actions>
        <button class="btn btn-secondary btn-sm gap-2" @click="$emit('refresh')">
          {{ $t('core.refresh') }}
        </button>
      </template>
    </PanelHeader>

    <div class="space-y-4">
      <div class="rounded-xl bg-sand-50 p-3">
        <p class="text-sm text-ink-700">
          {{ coreCurrent ? $t('core.current', { version: coreCurrent }) : $t('core.default') }}
        </p>
        <p class="help-text mt-1">
          {{ coreOperationText || $t('core.operation_idle') }}
        </p>
      </div>

      <div class="rounded-xl border border-sand-200 bg-white/85 p-3">
        <div class="flex items-start justify-between gap-3">
          <div>
            <p class="label">{{ $t('core.latest_stable') }}</p>
            <p class="font-mono text-sm text-ink-800">
              {{ coreLatestStable || $t('core.latest_unknown') }}
            </p>
            <p v-if="coreLatestStableDate" class="help-text">
              {{ $t('core.latest_date', { date: formatReleaseDate(coreLatestStableDate) }) }}
            </p>
          </div>
          <button class="btn btn-xs btn-secondary" @click="$emit('refresh-latest')">
            {{ $t('core.refresh_latest') }}
          </button>
        </div>
        <div class="mt-3 flex flex-wrap gap-2">
          <button class="btn btn-sm btn-primary" :disabled="!coreLatestStable" @click="$emit('update-stable')">
            {{ $t('core.update_stable') }}
          </button>
          <button class="btn btn-sm btn-secondary" :disabled="!coreLatestStable" @click="downloadLatestStable">
            {{ $t('core.download_stable') }}
          </button>
        </div>
      </div>

      <div class="rounded-xl border border-sand-200 bg-white/85 p-3">
        <p class="label">{{ $t('core.download_version_label') }}</p>
        <div class="mt-2 flex items-center gap-2">
          <input
            v-model="manualVersion"
            type="text"
            class="input w-full"
            :placeholder="$t('core.download_version_placeholder')"
          />
          <button class="btn btn-sm btn-secondary shrink-0" @click="downloadManualVersion">
            {{ $t('core.download') }}
          </button>
        </div>
      </div>

      <div class="max-h-75 overflow-y-auto">
        <ul class="space-y-2">
          <li
            v-for="version in coreVersions"
            :key="version"
            class="flex items-center justify-between rounded-xl border border-sand-200 bg-white p-3 transition-colors hover:border-primary-200"
          >
            <div>
              <p class="font-mono text-sm font-medium">{{ version }}</p>
              <p class="text-xs text-ink-500">
                {{ version === coreCurrent ? $t('core.active') : $t('core.switchable') }}
              </p>
            </div>
            <button
              class="btn btn-xs"
              :class="version === coreCurrent ? 'btn-secondary' : 'btn-primary'"
              :disabled="version === coreCurrent"
              @click="$emit('activate', version)"
            >
              {{ version === coreCurrent ? $t('core.status_current') : $t('core.status_use') }}
            </button>
          </li>
        </ul>
        <div v-if="coreVersions.length === 0" class="py-4 text-center empty-text">
          {{ $t('core.empty') }}
        </div>
      </div>
    </div>
  </PanelCard>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import PanelCard from './PanelCard.vue';
import PanelHeader from './PanelHeader.vue';
import PanelTitle from './PanelTitle.vue';

const props = defineProps<{
  coreVersions: string[];
  coreCurrent: string | null;
  coreLatestStable: string | null;
  coreLatestStableDate: string | null;
  coreOperationText: string;
}>();

const emit = defineEmits<{
  (e: 'refresh'): void;
  (e: 'refresh-latest'): void;
  (e: 'activate', version: string): void;
  (e: 'download', version: string): void;
  (e: 'update-stable'): void;
}>();

const manualVersion = ref('');

watch(
  () => props.coreLatestStable,
  (value) => {
    if (!manualVersion.value && value) {
      manualVersion.value = value;
    }
  },
  { immediate: true },
);

function downloadLatestStable() {
  if (!props.coreLatestStable) {
    return;
  }
  emit('download', props.coreLatestStable);
}

function downloadManualVersion() {
  const version = manualVersion.value.trim();
  if (!version) {
    return;
  }
  emit('download', version);
}

function formatReleaseDate(value: string) {
  if (!value) {
    return value;
  }
  const parsed = new Date(value);
  if (Number.isNaN(parsed.getTime())) {
    return value;
  }
  return parsed.toLocaleString();
}
</script>
