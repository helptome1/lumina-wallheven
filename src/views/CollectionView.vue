<script setup lang="ts">
import { useCollectionStore } from '@/stores/collection'
import WallpaperGrid from '@/components/wallpaper/WallpaperGrid.vue'
import EmptyState from '@/components/common/EmptyState.vue'

const collection = useCollectionStore()
</script>

<template>
  <div class="flex flex-col h-full">
    <div class="px-6 py-4 flex items-center justify-between">
      <div>
        <h2 class="display-italic text-accent text-2xl">我的收藏</h2>
        <p class="text-xs text-subtle mt-1">{{ collection.items.length }} 张壁纸</p>
      </div>
    </div>
    <div class="flex-1 overflow-hidden">
      <template v-if="collection.items.length > 0">
        <WallpaperGrid
          :wallpapers="collection.items"
          :loading="false"
          :has-more="false"
        />
      </template>
      <EmptyState
        v-else
        title="还没有收藏"
        description="点击壁纸上的心形图标即可收藏"
      />
    </div>
  </div>
</template>
