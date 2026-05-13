<script setup lang="ts">
import type { WallpaperData } from '@/types/wallhaven'
import { Heart, Download, Eye } from 'lucide-vue-next'
import { useCollectionStore } from '@/stores/collection'
import { useDownloadStore } from '@/stores/download'
import { inject } from 'vue'

const props = defineProps<{ data: WallpaperData }>()
const collection = useCollectionStore()
const downloadStore = useDownloadStore()
const openDetail = inject<(data: WallpaperData, list?: WallpaperData[]) => void>('openDetail')!

const handleFavorite = (e: Event) => {
  e.stopPropagation()
  collection.toggle(props.data)
}

const handleDownload = (e: Event) => {
  e.stopPropagation()
  downloadStore.startDownload(props.data)
}

const isFavorited = collection.isCollected(props.data.id)

const purityBadge = (p: string) => {
  switch (p) {
    case 'sfw': return { text: 'SFW', class: 'badge-sfw' }
    case 'sketchy': return { text: 'SKETCHY', class: 'badge-sketchy' }
    default: return { text: 'NSFW', class: 'badge-nsfw' }
  }
}

const purity = purityBadge(props.data.purity)
</script>

<template>
  <div
    class="wallpaper-card group relative overflow-hidden rounded-2xl cursor-pointer break-inside-avoid mb-4"
    @click="openDetail(data)"
  >
    <!-- Border glow ring -->
    <div class="absolute inset-0 rounded-2xl ring-1 ring-white/[0.05] pointer-events-none z-10" />

    <!-- Thumbnail -->
    <img
      :src="data.thumbs.large || data.thumbs.original"
      :alt="`Wallpaper ${data.id}`"
      loading="lazy"
      class="w-full h-auto block transition-transform duration-700 ease-out group-hover:scale-[1.04]"
      draggable="false"
    />

    <!-- Hover overlay -->
    <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-400 ease-out z-20">
      <!-- Aurora tinted gradient -->
      <div class="absolute inset-0 bg-gradient-to-t from-[#07071A]/95 via-[#0C0C24]/40 to-transparent" />

      <!-- Purple glow at bottom -->
      <div class="absolute bottom-0 left-0 right-0 h-1/2 bg-gradient-to-t from-violet-500/10 to-transparent" />

      <!-- Actions row -->
      <div class="absolute bottom-0 left-0 right-0 p-3.5 flex items-end justify-between">
        <div class="flex flex-col gap-0.5">
          <span class="text-white/90 text-xs font-semibold tracking-wide">{{ data.resolution }}</span>
          <span class="text-white/40 text-[11px] flex items-center gap-1.5">
            <Eye :size="11" />
            {{ data.views?.toLocaleString() || '—' }}
          </span>
        </div>

        <div class="flex gap-2" @click.stop>
          <!-- Favorite button -->
          <button
            @click="handleFavorite"
            class="action-btn group/btn"
            :class="isFavorited
              ? '!bg-rose-500/20 !text-rose-400 !border-rose-500/20 hover:!bg-rose-500/30'
              : ''"
          >
            <Heart
              :size="15"
              :fill="isFavorited ? 'currentColor' : 'none'"
              class="transition-transform duration-300 group-hover/btn:scale-110"
            />
          </button>

          <!-- Download button -->
          <button
            @click="handleDownload"
            class="action-btn group/btn"
          >
            <Download :size="15" class="transition-transform duration-300 group-hover/btn:scale-110" />
          </button>
        </div>
      </div>
    </div>

    <!-- Purity badge -->
    <span
      v-if="data.purity !== 'sfw'"
      class="absolute top-2.5 right-2.5 z-30 px-2 py-0.5 rounded-md font-semibold text-[10px] uppercase tracking-widest backdrop-blur-sm"
      :class="purity.class"
    >
      {{ purity.text }}
    </span>
  </div>
</template>

<style scoped>
.wallpaper-card {
  animation: card-in 0.5s ease-out both;
  animation-delay: calc(var(--index, 0) * 25ms);
}

@keyframes card-in {
  from {
    opacity: 0;
    transform: translateY(20px) scale(0.96);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.action-btn {
  padding: 0.55rem;
  border-radius: 0.875rem;
  backdrop-filter: blur(16px);
  -webkit-backdrop-filter: blur(16px);
  background: rgba(255, 255, 255, 0.05);
  color: rgba(255, 255, 255, 0.4);
  border: 1px solid rgba(255, 255, 255, 0.06);
  transition: all 0.3s ease;
  cursor: pointer;
}
.action-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  color: rgba(255, 255, 255, 0.9);
  border-color: rgba(255, 255, 255, 0.12);
  box-shadow: 0 0 24px var(--accent-glow);
}

.badge-sfw {
  background: rgba(45, 212, 191, 0.2);
  color: var(--accent-2);
  border: 1px solid rgba(45, 212, 191, 0.2);
}
.badge-sketchy {
  background: rgba(245, 158, 11, 0.2);
  color: var(--warm);
  border: 1px solid rgba(245, 158, 11, 0.2);
}
.badge-nsfw {
  background: rgba(248, 113, 113, 0.2);
  color: var(--error);
  border: 1px solid rgba(248, 113, 113, 0.2);
}
</style>
