<script setup lang="ts">
import { ref, provide } from 'vue'
import { RouterView } from 'vue-router'
import Titlebar from './Titlebar.vue'
import Sidebar from './Sidebar.vue'
import type { WallpaperData } from '@/types/wallhaven'

const detailTarget = ref<WallpaperData | null>(null)
const detailList = ref<WallpaperData[]>([])
const detailVisible = ref(false)
const detailIndex = ref(0)

const openDetail = (data: WallpaperData, list?: WallpaperData[]) => {
  detailTarget.value = data
  detailVisible.value = true
  if (list && list.length > 0) {
    detailList.value = list
    detailIndex.value = list.findIndex(item => item.id === data.id)
  } else {
    detailList.value = []
    detailIndex.value = 0
  }
}

const closeDetail = () => {
  detailVisible.value = false
  detailTarget.value = null
}

const nextDetail = () => {
  if (detailList.value.length > 0 && detailIndex.value < detailList.value.length - 1) {
    detailIndex.value++
    detailTarget.value = detailList.value[detailIndex.value]
  }
}

const prevDetail = () => {
  if (detailList.value.length > 0 && detailIndex.value > 0) {
    detailIndex.value--
    detailTarget.value = detailList.value[detailIndex.value]
  }
}

provide('openDetail', openDetail)
provide('closeDetail', closeDetail)
</script>

<template>
  <div class="app-shell flex h-screen w-screen overflow-hidden aurora-bg">
    <Titlebar />

    <div class="flex flex-1 pt-9">
      <Sidebar />

      <!-- Main content area -->
      <main class="flex-1 overflow-hidden relative z-10">
        <RouterView v-slot="{ Component }">
          <transition name="page-fade" mode="out-in">
            <component :is="Component" />
          </transition>
        </RouterView>
      </main>
    </div>

    <!-- Wallpaper Detail Lightbox -->
    <template v-if="detailVisible && detailTarget">
      <component
        :is="() => import('@/components/wallpaper/WallpaperDetail.vue')"
        :data="detailTarget"
        :has-prev="detailIndex > 0"
        :has-next="detailIndex < detailList.length - 1"
        @close="closeDetail"
        @prev="prevDetail"
        @next="nextDetail"
      />
    </template>
  </div>
</template>

<style scoped>
.app-shell {
  background:
    radial-gradient(ellipse 80% 60% at 10% 10%, rgba(139, 92, 246, 0.10) 0%, transparent 50%),
    radial-gradient(ellipse 60% 80% at 90% 90%, rgba(45, 212, 191, 0.07) 0%, transparent 50%),
    radial-gradient(ellipse 50% 40% at 50% 50%, rgba(99, 102, 241, 0.04) 0%, transparent 60%),
    var(--bg-base);
}

.page-fade-enter-active,
.page-fade-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}
.page-fade-enter-from {
  opacity: 0;
  transform: translateY(8px);
}
.page-fade-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}
</style>
