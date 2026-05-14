<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useDownloadStore } from '@/stores/download'
import EmptyState from '@/components/common/EmptyState.vue'

const store = useDownloadStore()

const activeDownloads = computed(() => store.getActive())
const completedDownloads = computed(() => store.getCompleted())

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

function formatSpeed(bytes: number): string {
  return formatBytes(bytes) + '/s'
}

function progressPercent(received: number, total: number): number {
  if (total === 0) return 0
  return Math.min((received / total) * 100, 100)
}

onMounted(() => {
  store.setupListeners()
})
</script>

<template>
  <div class="flex flex-col h-full">
    <!-- Page Header -->
    <div class="h-20 flex items-center px-10 m-4 rounded-2xl border border-black/5 bg-white/40 backdrop-blur-[40px]">
      <div class="flex items-center gap-3">
        <span class="material-symbols-outlined text-primary text-[28px]">download</span>
        <div>
          <h2 class="text-headline-lg text-on-surface">Downloads</h2>
          <p class="text-body-sm text-on-surface-variant">{{ store.items.length }} items</p>
        </div>
      </div>
    </div>

    <div class="flex-1 overflow-y-auto px-10 pb-8 space-y-3 custom-scrollbar">
      <!-- Active downloads -->
      <div
        v-for="item in activeDownloads"
        :key="item.id"
        class="p-4 rounded-2xl border border-black/5 bg-white/60 backdrop-blur-sm"
      >
        <div class="flex items-center gap-4">
          <img
            v-if="item.thumbnail"
            :src="item.thumbnail"
            class="w-14 h-9 rounded-lg object-cover flex-shrink-0 border border-black/5"
          />
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium text-on-surface truncate">{{ item.file_name || item.id }}</p>
            <div class="flex items-center gap-2 mt-1">
              <span class="inline-block w-3 h-3 rounded-full border-2 border-primary border-t-transparent animate-spin" />
              <span class="text-xs text-on-surface-variant">
                {{ formatBytes(item.received_bytes) }} / {{ formatBytes(item.total_bytes) }}
              </span>
              <span class="text-xs text-on-surface-variant/50">{{ formatSpeed(item.speed_bytes) }}</span>
            </div>
            <div class="mt-2.5 h-1 bg-black/5 rounded-full overflow-hidden">
              <div
                class="h-full rounded-full transition-all duration-300 bg-primary"
                :style="{ width: `${progressPercent(item.received_bytes, item.total_bytes)}%` }"
              />
            </div>
          </div>
          <button
            @click="store.cancelDownload(item.id)"
            class="p-2 rounded-xl text-on-surface-variant/30 hover:text-error hover:bg-error/5 transition-all duration-200"
          >
            <span class="material-symbols-outlined text-[18px]">close</span>
          </button>
        </div>
      </div>

      <!-- Completed downloads -->
      <div
        v-for="item in completedDownloads"
        :key="item.id"
        class="p-4 rounded-2xl border border-black/5 bg-white/60 backdrop-blur-sm"
      >
        <div class="flex items-center gap-4">
          <img
            v-if="item.thumbnail"
            :src="item.thumbnail"
            class="w-14 h-9 rounded-lg object-cover flex-shrink-0 border border-black/5"
          />
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium text-on-surface truncate">{{ item.file_name || item.id }}</p>
            <p class="text-xs text-on-surface-variant mt-0.5">{{ item.resolution }}</p>
          </div>
          <span class="material-symbols-outlined text-primary text-[20px]">check_circle</span>
        </div>
      </div>

      <!-- Empty -->
      <div v-if="store.items.length === 0" class="flex-1 flex items-center justify-center">
        <EmptyState
          title="No downloads yet"
          description="Downloaded wallpapers will appear here"
        />
      </div>
    </div>
  </div>
</template>
