<script setup lang="ts">
import { ref, watch, onUnmounted, onMounted } from 'vue'
import type { WallpaperData } from '@/types/wallhaven'
import { wallhavenApi } from '@/api/wallhaven'
import { useCollectionStore } from '@/stores/collection'
import { useDownloadStore } from '@/stores/download'

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
const zoomed = ref(false)

async function loadImage(data: WallpaperData) {
  loading.value = true
  zoomed.value = false
  fullImageUrl.value = wallhavenApi.getFullImageUrl(data.path)
}

watch(() => props.data, (data) => {
  loadImage(data)
}, { immediate: true })

const isFavorited = collection.isCollected(props.data.id)

const favIconStyle = {
  fontVariationSettings: "'FILL' 1, 'wght' 400, 'GRAD' 0, 'opsz' 24",
}
const defaultIconStyle = {
  fontVariationSettings: "'FILL' 0, 'wght' 400, 'GRAD' 0, 'opsz' 24",
}

function handleFavorite() {
  collection.toggle(props.data)
}

function handleDownload() {
  downloadStore.startDownload(props.data)
}

function handleImageLoad() {
  loading.value = false
}

function onKeydown(e: KeyboardEvent) {
  if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return

  switch (e.key) {
    case 'Escape': emit('close'); break
    case 'ArrowLeft': if (props.hasPrev) emit('prev'); break
    case 'ArrowRight': if (props.hasNext) emit('next'); break
  }
}

function toggleZoom() {
  zoomed.value = !zoomed.value
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

// Category display name
const categoryName = () => {
  const map: Record<string, string> = {
    '100': 'Anime',
    '010': 'Anime',
    '001': 'People',
    '111': 'General',
    '101': 'Anime',
    '011': 'Anime',
    '110': 'General',
  }
  return map[props.data.category] || props.data.category || 'General'
}

// Tags to display
const displayTags = () => {
  if (props.data.tags && props.data.tags.length > 0) {
    return props.data.tags.slice(0, 6).map(t => t.name)
  }
  return props.data.colors?.slice(0, 4).map(c => `#${c}`) || []
}
</script>

<template>
  <Teleport to="body">
    <transition name="detail-fade">
      <div
        class="fixed inset-0 z-[100] flex items-center justify-center p-6 md:p-12 modal-overlay"
        @click.self="emit('close')"
      >
        <div
          class="max-w-6xl w-full max-h-[92%] light-glass-modal soft-glow overflow-hidden flex flex-col md:flex-row relative rounded-2xl shadow-xl"
        >
          <!-- macOS-style window dots -->
          <div class="absolute top-6 left-6 flex gap-2 z-50">
            <button @click="emit('close')" class="w-3 h-3 rounded-full bg-[#FF5F57] shadow-inner hover:brightness-90 transition-all" />
            <div class="w-3 h-3 rounded-full bg-[#FEBC2E] shadow-inner" />
            <div class="w-3 h-3 rounded-full bg-[#28C840] shadow-inner" />
          </div>

          <!-- Navigation arrows -->
          <button
            v-if="hasPrev"
            @click.stop="emit('prev')"
            class="absolute left-4 top-1/2 -translate-y-1/2 z-50 w-10 h-10 rounded-full bg-black/10 backdrop-blur-md border border-black/5 flex items-center justify-center hover:bg-black/20 transition-all text-on-surface-variant"
          >
            <span class="material-symbols-outlined">chevron_left</span>
          </button>
          <button
            v-if="hasNext"
            @click.stop="emit('next')"
            class="absolute right-[420px] top-1/2 -translate-y-1/2 z-50 w-10 h-10 rounded-full bg-black/10 backdrop-blur-md border border-black/5 flex items-center justify-center hover:bg-black/20 transition-all text-on-surface-variant"
          >
            <span class="material-symbols-outlined">chevron_right</span>
          </button>

          <!-- Left: Image Preview -->
          <div class="flex-1 bg-black/5 flex items-center justify-center p-4 relative">
            <div class="relative w-full h-full rounded-2xl overflow-hidden shadow-2xl border border-black/5 group">
              <!-- Loading -->
              <div
                v-if="loading"
                class="absolute inset-0 flex items-center justify-center bg-black/5"
              >
                <div class="flex gap-2">
                  <span v-for="i in 3" :key="i" class="w-2 h-2 rounded-full bg-primary/40 animate-bounce" :style="{ animationDelay: `${i * 0.15}s` }" />
                </div>
              </div>

              <img
                v-show="!loading"
                :src="fullImageUrl"
                :alt="`Wallpaper ${data.id}`"
                class="w-full h-full object-cover transition-transform duration-500 cursor-pointer"
                :class="{ 'scale-150': zoomed }"
                draggable="false"
                @load="handleImageLoad"
                @click="toggleZoom"
              />

              <!-- Image controls -->
              <div class="absolute bottom-6 right-6 flex gap-3">
                <button
                  @click="toggleZoom"
                  class="bg-black/20 backdrop-blur-xl hover:bg-black/40 p-2.5 rounded-xl text-white transition-all border border-white/20"
                >
                  <span class="material-symbols-outlined">{{ zoomed ? 'zoom_out' : 'zoom_in' }}</span>
                </button>
              </div>
            </div>
          </div>

          <!-- Right: Detail Panel -->
          <div class="w-full md:w-[400px] p-8 flex flex-col gap-8 bg-white/10 backdrop-blur-2xl border-l border-black/5 overflow-y-auto">
            <!-- Title & Author -->
            <div>
              <h2 class="font-headline-lg text-headline-lg text-modal-primary mb-2">
                Wallpaper {{ data.id }}
              </h2>
              <p class="font-body-md text-modal-secondary flex items-center gap-2">
                <span class="material-symbols-outlined text-[18px] text-primary">palette</span>
                By {{ data.uploader?.username || 'wallhaven' }}
              </p>
            </div>

            <!-- Resolution & Category -->
            <div class="grid grid-cols-2 gap-4">
              <div class="bg-black/5 p-4 rounded-2xl border border-black/5">
                <span class="block font-label-caps text-modal-secondary mb-1">RESOLUTION</span>
                <span class="font-headline-md text-modal-primary">{{ data.resolution }}</span>
              </div>
              <div class="bg-black/5 p-4 rounded-2xl border border-black/5">
                <span class="block font-label-caps text-modal-secondary mb-1">CATEGORY</span>
                <span class="font-headline-md text-modal-primary">{{ categoryName() }}</span>
              </div>
            </div>

            <!-- Tags -->
            <div v-if="displayTags().length > 0">
              <span class="block font-label-caps text-modal-secondary mb-4">METADATA TAGS</span>
              <div class="flex flex-wrap gap-2">
                <span
                  v-for="tag in displayTags()"
                  :key="tag"
                  class="px-4 py-1.5 bg-black/5 border border-black/5 rounded-full font-body-sm text-modal-primary"
                >{{ tag }}</span>
              </div>
            </div>

            <!-- Stats -->
            <div class="flex gap-4 text-modal-secondary font-body-sm">
              <span class="flex items-center gap-1">
                <span class="material-symbols-outlined text-[16px]">visibility</span>
                {{ data.views?.toLocaleString() || 0 }}
              </span>
              <span class="flex items-center gap-1">
                <span class="material-symbols-outlined text-[16px]">favorite</span>
                {{ data.favorites?.toLocaleString() || 0 }}
              </span>
            </div>

            <!-- Actions -->
            <div class="mt-auto flex flex-col gap-3">
              <button
                @click="handleDownload"
                class="w-full py-4 bg-[#FF7D4E] text-white rounded-2xl font-headline-md flex items-center justify-center gap-3 hover:brightness-110 transition-all shadow-lg shadow-[#FF7D4E]/20"
              >
                <span class="material-symbols-outlined">download</span>
                Download Wallpaper
              </button>
              <div class="flex gap-3">
                <button
                  @click="handleFavorite"
                  class="flex-1 py-4 bg-black/5 border border-black/5 rounded-2xl text-modal-primary font-headline-md flex items-center justify-center gap-3 hover:bg-black/10 transition-all"
                  :class="{ '!bg-primary/10 !border-primary/20 !text-primary': isFavorited }"
                >
                  <span
                    class="material-symbols-outlined"
                    :class="{ 'text-[#FF7D4E]': !isFavorited }"
                    :style="isFavorited ? favIconStyle : defaultIconStyle"
                  >favorite</span>
                  {{ isFavorited ? 'Favorited' : 'Favorite' }}
                </button>
                <button
                  class="w-16 py-4 bg-black/5 border border-black/5 rounded-2xl text-modal-primary flex items-center justify-center hover:bg-black/10 transition-all"
                >
                  <span class="material-symbols-outlined">share</span>
                </button>
              </div>
            </div>

            <!-- Social proof -->
            <div class="pt-6 border-t border-black/5">
              <div class="flex items-center justify-between">
                <div class="flex -space-x-3">
                  <div class="w-10 h-10 rounded-full border-2 border-white/50 bg-primary-container shadow-lg" />
                  <div class="w-10 h-10 rounded-full border-2 border-white/50 bg-secondary-container shadow-lg" />
                  <div class="w-10 h-10 rounded-full border-2 border-white/50 bg-orange-400/60 shadow-lg" />
                </div>
                <span class="font-body-sm text-modal-secondary">Used by 1.2k creatives</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </transition>
  </Teleport>
</template>

<style scoped>
.detail-fade-enter-active {
  transition: opacity 0.3s ease-out;
}
.detail-fade-enter-from {
  opacity: 0;
}
.detail-fade-leave-active {
  transition: opacity 0.2s ease-out;
}
.detail-fade-leave-to {
  opacity: 0;
}
</style>
