<script setup lang="ts">
import type { WallpaperData } from '@/types/wallhaven'
import { useCollectionStore } from '@/stores/collection'
import { useDownloadStore } from '@/stores/download'
import { inject } from 'vue'

const props = defineProps<{ data: WallpaperData }>()
const collection = useCollectionStore()
const downloadStore = useDownloadStore()
const openDetail = inject<(data: WallpaperData, list?: WallpaperData[]) => void>('openDetail')!

const isFavorited = collection.isCollected(props.data.id)

const favIconStyle = {
  fontVariationSettings: "'FILL' 1, 'wght' 400, 'GRAD' 0, 'opsz' 24",
}
const defaultIconStyle = {
  fontVariationSettings: "'FILL' 0, 'wght' 400, 'GRAD' 0, 'opsz' 24",
}

const handleFavorite = (e: Event) => {
  e.stopPropagation()
  collection.toggle(props.data)
}

const handleDownload = (e: Event) => {
  e.stopPropagation()
  downloadStore.startDownload(props.data)
}
</script>

<template>
  <div
    class="group relative aspect-[4/3] overflow-hidden rounded-[2rem] border border-black/5 shadow-xl transition-all duration-500 hover:scale-[1.02] hover:border-primary/40 cursor-pointer"
    @click="openDetail(data)"
  >
    <!-- Thumbnail -->
    <img
      :src="data.thumbs.large || data.thumbs.original"
      :alt="`Wallpaper ${data.id}`"
      loading="lazy"
      class="w-full h-full object-cover"
      draggable="false"
      referrerpolicy="no-referrer"
    />

    <!-- Hover Overlay -->
    <div
      class="absolute inset-0 wallpaper-card-overlay opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex flex-col justify-end p-6 backdrop-blur-[40px]"
    >
      <div class="flex justify-between items-center text-on-surface">
        <span class="text-label-caps text-on-surface-variant font-bold">{{ data.resolution }}</span>
        <div class="flex gap-2" @click.stop>
          <!-- Favorite Button -->
          <button
            @click="handleFavorite"
            class="w-10 h-10 rounded-full flex items-center justify-center transition-all"
            :class="isFavorited
              ? 'bg-primary text-on-primary shadow-lg'
              : 'bg-white/20 backdrop-blur-md border border-white/40 hover:bg-primary hover:text-on-primary'"
          >
            <span
              class="material-symbols-outlined text-[20px]"
              :style="isFavorited ? favIconStyle : defaultIconStyle"
            >favorite</span>
          </button>
          <!-- Download Button -->
          <button
            @click="handleDownload"
            class="w-10 h-10 rounded-full bg-primary text-on-primary flex items-center justify-center shadow-lg hover:brightness-110 transition-all"
          >
            <span class="material-symbols-outlined text-[20px]">download</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
