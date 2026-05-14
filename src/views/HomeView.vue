<script setup lang="ts">
import { watch } from 'vue'
import WallpaperGrid from '@/components/wallpaper/WallpaperGrid.vue'
import { useBrowsePage } from '@/composables/useBrowsePage'
import { useFilterStore } from '@/stores/filter'

const filterStore = useFilterStore()

const { store, hasMore, target, onSearch, onDownload } = useBrowsePage({
  categories: '111',
  purity: filterStore.params.purity || '100',
  sorting: filterStore.params.sorting || 'toplist',
  atleast: filterStore.params.atleast || '3840x2160',
  q: filterStore.params.q || '',
})

// React to filter store changes
watch(
  () => [filterStore.params.purity, filterStore.params.sorting, filterStore.params.atleast, filterStore.params.q],
  () => {
    onSearch(filterStore.getSnapshot())
  },
)
</script>

<template>
  <div class="flex flex-col h-full">
    <WallpaperGrid
      :wallpapers="store.wallpapers"
      :loading="store.loading"
      :has-more="hasMore"
      @download="onDownload"
    />
    <div ref="target" class="absolute bottom-0 left-0 right-0 h-1 pointer-events-none" />
  </div>
</template>
