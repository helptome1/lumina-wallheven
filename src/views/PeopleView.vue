<script setup lang="ts">
import { watch } from 'vue'
import WallpaperGrid from '@/components/wallpaper/WallpaperGrid.vue'
import { useBrowsePage } from '@/composables/useBrowsePage'
import { useFilterStore } from '@/stores/filter'

const filterStore = useFilterStore()

const { store, hasMore, onLoadMore, onSearch, onDownload } = useBrowsePage({
  categories: '001',
  purity: filterStore.params.purity || '100',
  sorting: filterStore.params.sorting || 'toplist',
  atleast: filterStore.params.atleast || '3840x2160',
  q: filterStore.params.q || '',
})

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
      @load-more="onLoadMore"
      @download="onDownload"
    />
  </div>
</template>
