<script setup lang="ts">
import SearchBar from '@/components/wallpaper/SearchBar.vue'
import WallpaperGrid from '@/components/wallpaper/WallpaperGrid.vue'
import { useBrowsePage } from '@/composables/useBrowsePage'

const { store, hasMore, target, onSearch, onDownload } = useBrowsePage({
  categories: '010',
  purity: '100',
  sorting: 'toplist',
})
</script>

<template>
  <div class="flex flex-col h-full">
    <SearchBar @search="onSearch" />
    <div class="flex-1 overflow-hidden relative">
      <WallpaperGrid
        :wallpapers="store.wallpapers"
        :loading="store.loading"
        :has-more="hasMore"
        @download="onDownload"
      />
      <div ref="target" class="absolute bottom-0 left-0 right-0 h-1 pointer-events-none" />
    </div>
  </div>
</template>
