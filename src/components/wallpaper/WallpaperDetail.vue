<script setup lang="ts">
import { ref, watch, onUnmounted, onMounted } from 'vue'
import type { WallpaperData } from '@/types/wallhaven'
import { wallhavenApi } from '@/api/wallhaven'
import { useCollectionStore } from '@/stores/collection'
import { useDownloadStore } from '@/stores/download'
import { X, Heart, Download, ZoomIn, ZoomOut, RotateCw, ChevronLeft, ChevronRight } from 'lucide-vue-next'

const props = defineProps<{
  data: WallpaperData
  hasPrev: boolean
  hasNext: boolean
}>()

const emit = defineEmits<{
  close: []
  prev: []
  next: []
}>()

const collection = useCollectionStore()
const downloadStore = useDownloadStore()

const fullImageUrl = ref('')
const loading = ref(true)
const zoom = ref(1)
const rotation = ref(0)

async function loadImage(data: WallpaperData) {
  loading.value = true
  zoom.value = 1
  rotation.value = 0
  fullImageUrl.value = wallhavenApi.getFullImageUrl(data.path)
}

watch(() => props.data, (data) => {
  loadImage(data)
}, { immediate: true })

function onKeydown(e: KeyboardEvent) {
  switch (e.key) {
    case 'Escape': emit('close'); break
    case 'ArrowLeft': if (props.hasPrev) emit('prev'); break
    case 'ArrowRight': if (props.hasNext) emit('next'); break
    case '+': case '=': zoom.value = Math.min(zoom.value + 0.25, 5); break
    case '-': zoom.value = Math.max(zoom.value - 0.25, 0.25); break
    case 'r': case 'R': rotation.value = (rotation.value + 90) % 360; break
    case '0': zoom.value = 1; rotation.value = 0; break
  }
}

let scrollDisabled = false
onMounted(() => {
  window.addEventListener('keydown', onKeydown)
  if (!scrollDisabled) {
    document.body.style.overflow = 'hidden'
    scrollDisabled = true
  }
})
onUnmounted(() => {
  window.removeEventListener('keydown', onKeydown)
  document.body.style.overflow = ''
  scrollDisabled = false
})

const isFavorited = collection.isCollected(props.data.id)

function handleFavorite() {
  collection.toggle(props.data)
}

function handleDownload() {
  downloadStore.startDownload(props.data)
}

function handleImageLoad() {
  loading.value = false
}
</script>

<template>
  <Teleport to="body">
    <transition name="detail-fade">
      <div
        class="fixed inset-0 z-[99999] flex items-center justify-center"
        @click.self="emit('close')"
      >
        <!-- Backdrop with aurora -->
        <div class="absolute inset-0 bg-[#07071A]/92 backdrop-blur-3xl" />
        <div class="absolute inset-0 bg-gradient-to-br from-violet-500/[0.04] via-transparent to-teal-400/[0.04]" />

        <!-- Close button -->
        <button
          @click="emit('close')"
          class="absolute top-5 right-5 z-20 p-2.5 rounded-2xl bg-white/[0.03] hover:bg-white/[0.07] text-white/30 hover:text-white border border-white/[0.06] hover:border-white/[0.12] transition-all duration-300 backdrop-blur-xl"
        >
          <X :size="20" />
        </button>

        <!-- Navigation arrows -->
        <button
          v-if="hasPrev"
          @click.stop="emit('prev')"
          class="absolute left-5 top-1/2 -translate-y-1/2 z-20 p-3.5 rounded-2xl bg-white/[0.02] hover:bg-white/[0.06] text-white/20 hover:text-white border border-white/[0.04] hover:border-white/[0.1] transition-all duration-300 backdrop-blur-xl"
        >
          <ChevronLeft :size="28" />
        </button>
        <button
          v-if="hasNext"
          @click.stop="emit('next')"
          class="absolute right-5 top-1/2 -translate-y-1/2 z-20 p-3.5 rounded-2xl bg-white/[0.02] hover:bg-white/[0.06] text-white/20 hover:text-white border border-white/[0.04] hover:border-white/[0.1] transition-all duration-300 backdrop-blur-xl"
        >
          <ChevronRight :size="28" />
        </button>

        <!-- Image container -->
        <div class="relative max-w-[90vw] max-h-[85vh] z-10 flex items-center justify-center">
          <!-- Loading -->
          <div
            v-if="loading"
            class="absolute inset-0 flex flex-col items-center justify-center gap-3"
          >
            <div class="w-8 h-8 border-2 border-violet-400/30 border-t-violet-400 rounded-full animate-spin" />
            <span class="text-white/10 text-xs">加载中…</span>
          </div>

          <!-- Image -->
          <img
            v-show="!loading"
            :src="fullImageUrl"
            :alt="`Wallpaper ${data.id}`"
            class="max-w-full max-h-[85vh] object-contain rounded-2xl shadow-[0_0_120px_rgba(139,92,246,0.1)] transition-all duration-300 ease-out select-none"
            :class="{ 'cursor-zoom-in': zoom === 1 }"
            :style="{
              transform: `scale(${zoom}) rotate(${rotation}deg)`,
            }"
            draggable="false"
            @load="handleImageLoad"
          />
        </div>

        <!-- Info panel -->
        <div class="absolute left-8 bottom-28 z-20">
          <div class="px-4 py-3 rounded-2xl bg-white/[0.02] backdrop-blur-2xl border border-white/[0.05]">
            <p class="text-sm font-medium text-white/80">{{ data.id }}</p>
            <p class="text-xs text-white/20 mt-1">
              {{ data.resolution }}
              <span class="mx-1.5 opacity-20">&middot;</span>
              {{ data.file_type?.toUpperCase() }}
              <span v-if="data.file_size" class="mx-1.5 opacity-20">&middot;</span>
              <span v-if="data.file_size">{{ (data.file_size / 1024 / 1024).toFixed(1) }}MB</span>
            </p>
          </div>
        </div>

        <!-- Control toolbar -->
        <div class="absolute bottom-7 left-1/2 -translate-x-1/2 z-20 flex items-center gap-1.5 px-4 py-3 rounded-2xl bg-white/[0.03] backdrop-blur-2xl border border-white/[0.05]">
          <button @click="zoom = Math.max(zoom - 0.25, 0.25)" class="toolbar-btn" title="缩小 (-)">
            <ZoomOut :size="17" />
          </button>
          <span class="text-xs text-white/35 w-12 text-center tabular-nums font-medium">
            {{ Math.round(zoom * 100) }}%
          </span>
          <button @click="zoom = Math.min(zoom + 0.25, 5)" class="toolbar-btn" title="放大 (+)">
            <ZoomIn :size="17" />
          </button>

          <div class="w-px h-6 bg-white/[0.06] mx-2" />

          <button @click="rotation = (rotation + 90) % 360" class="toolbar-btn" title="旋转 (R)">
            <RotateCw :size="17" />
          </button>

          <div class="w-px h-6 bg-white/[0.06] mx-2" />

          <button
            @click="handleFavorite"
            class="toolbar-btn"
            :class="isFavorited ? '!text-rose-400 !bg-rose-500/10' : ''"
            :title="isFavorited ? '取消收藏' : '收藏'"
          >
            <Heart :size="17" :fill="isFavorited ? 'currentColor' : 'none'" />
          </button>

          <button @click="handleDownload" class="toolbar-btn" title="下载">
            <Download :size="17" />
          </button>
        </div>
      </div>
    </transition>
  </Teleport>
</template>

<style scoped>
.detail-fade-enter-active {
  transition: opacity 0.3s ease;
}
.detail-fade-leave-active {
  transition: opacity 0.2s ease;
}
.detail-fade-enter-from,
.detail-fade-leave-to {
  opacity: 0;
}
.toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0.55rem;
  border: none;
  border-radius: 0.75rem;
  background: transparent;
  color: rgba(255, 255, 255, 0.3);
  cursor: pointer;
  transition: all 0.2s ease;
}
.toolbar-btn:hover {
  color: rgba(255, 255, 255, 0.85);
  background-color: rgba(139, 92, 246, 0.12);
}
</style>
