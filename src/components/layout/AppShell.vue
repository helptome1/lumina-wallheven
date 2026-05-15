<script setup lang="ts">
import { ref, provide } from 'vue'
import { RouterView } from 'vue-router'
import Sidebar from './Sidebar.vue'
import GlobalHeader from '@/components/wallpaper/GlobalHeader.vue'
import WallpaperDetail from '@/components/wallpaper/WallpaperDetail.vue'
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
  <div class="flex h-screen overflow-hidden text-on-surface bg-background">
    <!-- Background Layer (matching layout.html) -->
    <div class="fixed inset-0 z-0 overflow-hidden">
      <div class="w-full h-full bg-gradient-to-br from-primary-fixed/20 via-surface-bright/40 to-tertiary-fixed/20" />
    </div>

    <!-- Sidebar Navigation (fixed, matching layout.html) -->
    <Sidebar />

    <!-- Main Content Area (ml-[280px] matching layout.html sidebar width) -->
    <main class="flex-1 relative z-10 flex flex-col h-screen overflow-hidden">
      <!-- Global Header (SFW/Sketchy/NSFW, Sort/Res dropdowns, Search) -->
      <GlobalHeader />

      <!-- Page Content -->
      <RouterView v-slot="{ Component }">
        <transition name="page" mode="out-in">
          <component :is="Component" />
        </transition>
      </RouterView>

      <!-- Floating Action Button (FAB) -->
      <button
        class="fixed bottom-10 right-10 w-16 h-16 rounded-full bg-primary text-on-primary shadow-[0_0_40px_rgba(70,72,212,0.3)] flex items-center justify-center hover:scale-110 active:scale-95 transition-all duration-300 z-50"
      >
        <span class="material-symbols-outlined text-[32px]">add</span>
      </button>
    </main>

    <!-- Wallpaper Detail Lightbox -->
    <template v-if="detailVisible && detailTarget">
      <WallpaperDetail
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
