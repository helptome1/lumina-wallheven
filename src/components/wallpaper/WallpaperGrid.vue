<script setup lang="ts">
import type { WallpaperData } from '@/types/wallhaven'
import WallpaperCard from './WallpaperCard.vue'
import SkeletonGrid from '@/components/common/SkeletonGrid.vue'
import EmptyState from '@/components/common/EmptyState.vue'

defineProps<{
  wallpapers: WallpaperData[]
  loading: boolean
  hasMore: boolean
}>()

const emit = defineEmits<{
  loadMore: []
  download: [data: WallpaperData]
}>()
</script>

<template>
  <div class="wallpaper-grid-container h-full overflow-y-auto custom-scroll">
    <!-- Wallpapers -->
    <div
      v-if="wallpapers.length > 0"
      class="px-5 pb-8 pt-2"
    >
      <div class="columns-2 md:columns-3 lg:columns-4 xl:columns-5 2xl:columns-6 gap-4">
        <WallpaperCard
          v-for="(wallpaper, index) in wallpapers"
          :key="wallpaper.id"
          :data="wallpaper"
          :style="{ '--index': index % 30 }"
          @download="emit('download', wallpaper)"
        />
      </div>
    </div>

    <!-- Initial loading -->
    <SkeletonGrid v-if="loading && wallpapers.length === 0" />

    <!-- Load more spinner -->
    <div
      v-if="loading && wallpapers.length > 0"
      class="flex justify-center py-10"
    >
      <div class="flex items-center gap-3">
        <div class="w-5 h-5 border-2 border-violet-400/30 border-t-violet-400 rounded-full animate-spin" />
        <span class="text-white/15 text-sm">加载更多…</span>
      </div>
    </div>

    <!-- Empty state -->
    <EmptyState
      v-if="!loading && wallpapers.length === 0"
      title="暂无壁纸"
      description="尝试调整搜索条件或筛选器"
    />
  </div>
</template>

<style scoped>
.custom-scroll::-webkit-scrollbar {
  width: 5px;
}
.custom-scroll::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scroll::-webkit-scrollbar-thumb {
  background: rgba(167, 139, 250, 0.08);
  border-radius: 3px;
}
.custom-scroll::-webkit-scrollbar-thumb:hover {
  background: rgba(167, 139, 250, 0.18);
}
</style>
