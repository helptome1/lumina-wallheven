<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useDownloadStore } from '@/stores/download'
import { Download, CheckCircle, Loader2, X } from 'lucide-vue-next'

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
    <div class="px-6 py-4">
      <h2 class="display-italic text-accent text-2xl">下载中心</h2>
    </div>

    <div class="flex-1 overflow-y-auto px-6 pb-8 space-y-3">
      <!-- Active downloads -->
      <div
        v-for="item in activeDownloads"
        :key="item.id"
        class="glass-card p-4 rounded-2xl"
      >
        <div class="flex items-center gap-4">
          <img
            v-if="item.thumbnail"
            :src="item.thumbnail"
            class="w-14 h-9 rounded-lg object-cover flex-shrink-0 ring-1 ring-white/[0.04]"
          />
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium text-white/75 truncate">{{ item.file_name || item.id }}</p>
            <div class="flex items-center gap-2 mt-1">
              <Loader2 :size="12" class="animate-spin text-accent" />
              <span class="text-xs text-white/20">
                {{ formatBytes(item.received_bytes) }} / {{ formatBytes(item.total_bytes) }}
              </span>
              <span class="text-xs text-white/10">{{ formatSpeed(item.speed_bytes) }}</span>
            </div>
            <div class="mt-2.5 h-1 bg-white/[0.04] rounded-full overflow-hidden">
              <div
                class="h-full rounded-full transition-all duration-300"
                :style="{
                  width: `${progressPercent(item.received_bytes, item.total_bytes)}%`,
                  background: 'linear-gradient(90deg, #8B5CF6, #2DD4BF)'
                }"
              />
            </div>
          </div>
          <button
            @click="store.cancelDownload(item.id)"
            class="p-2 rounded-xl text-white/10 hover:text-rose-400 hover:bg-rose-400/10 transition-all duration-200"
          >
            <X :size="16" />
          </button>
        </div>
      </div>

      <!-- Completed downloads -->
      <div
        v-for="item in completedDownloads"
        :key="item.id"
        class="glass-card p-4 rounded-2xl"
      >
        <div class="flex items-center gap-4">
          <img
            v-if="item.thumbnail"
            :src="item.thumbnail"
            class="w-14 h-9 rounded-lg object-cover flex-shrink-0 ring-1 ring-white/[0.04]"
          />
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium text-white/75 truncate">{{ item.file_name || item.id }}</p>
            <p class="text-xs text-white/15 mt-0.5">{{ item.resolution }}</p>
          </div>
          <CheckCircle :size="18" class="text-accent-2 flex-shrink-0" />
        </div>
      </div>

      <!-- Empty -->
      <div
        v-if="store.items.length === 0"
        class="flex flex-col items-center justify-center h-full gap-5 py-20"
      >
        <div class="w-16 h-16 rounded-2xl bg-white/[0.02] border border-white/[0.04] flex items-center justify-center">
          <Download :size="28" class="text-white/10" />
        </div>
        <h3 class="text-base font-medium text-white/20">暂无下载</h3>
        <p class="text-sm text-white/10">下载壁纸后将在此处显示</p>
      </div>
    </div>
  </div>
</template>
