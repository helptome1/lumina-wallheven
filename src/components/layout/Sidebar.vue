<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'
import { computed } from 'vue'

const route = useRoute()
const router = useRouter()

interface NavItem {
  path: string
  label: string
  icon: string
}

const navItems: NavItem[] = [
  { path: '/hot', label: 'Explore', icon: 'explore' },
  { path: '/hot', label: 'Hot', icon: 'local_fire_department' },
  { path: '/anime', label: 'Anime', icon: 'auto_awesome' },
  { path: '/people', label: 'People', icon: 'groups' },
  { path: '/collection', label: 'Favorites', icon: 'favorite' },
  { path: '/downloads', label: 'Downloads', icon: 'download' },
]

const activePath = computed(() => route.path)

function isActive(path: string): boolean {
  return activePath.value === path
}

function navigate(path: string) {
  router.push(path)
}
</script>

<template>
  <aside
    class="w-sidebar-width h-screen sticky left-0 top-0 z-20 flex flex-col p-6 shadow-xl m-4 rounded-2xl border border-black/5 bg-white/40 backdrop-blur-[40px]"
  >
    <!-- Brand -->
    <div class="mb-10 px-2">
      <h1 class="text-headline-xl text-primary leading-tight">Lumina</h1>
      <p class="text-body-sm text-on-surface-variant font-medium">4K Creative Gallery</p>
    </div>

    <!-- Navigation -->
    <nav class="flex-1 space-y-1">
      <button
        v-for="item in navItems"
        :key="item.label"
        @click="navigate(item.path)"
        class="flex items-center gap-3 rounded-xl px-4 py-3 transition-all duration-300 w-full text-left"
        :class="isActive(item.path)
          ? 'bg-primary text-on-primary border border-primary/20 shadow-lg shadow-primary/20 font-bold'
          : 'text-on-surface-variant hover:text-on-surface hover:bg-black/5'"
      >
        <span class="material-symbols-outlined">{{ item.icon }}</span>
        <span class="text-body-md">{{ item.label }}</span>
      </button>
    </nav>

    <!-- Bottom Section -->
    <div class="mt-auto pt-6 border-t border-black/5">
      <button
        @click="navigate('/about')"
        class="flex items-center gap-3 text-on-surface-variant hover:text-on-surface hover:bg-black/5 rounded-xl px-4 py-3 transition-all mb-4 w-full text-left"
        :class="{ 'bg-primary text-on-primary border border-primary/20 shadow-lg shadow-primary/20 font-bold': isActive('/about') }"
      >
        <span class="material-symbols-outlined">info</span>
        <span class="text-body-md">About</span>
      </button>

      <!-- User Profile Card -->
      <div class="flex items-center gap-3 px-4 py-3 bg-surface-container-low rounded-2xl border border-black/5">
        <div class="w-10 h-10 rounded-full overflow-hidden border-2 border-primary/20 flex-shrink-0">
          <div class="w-full h-full bg-gradient-to-br from-primary to-primary-fixed-dim flex items-center justify-center">
            <span class="text-on-primary text-sm font-bold">E</span>
          </div>
        </div>
        <div class="flex flex-col min-w-0">
          <span class="text-body-sm font-bold text-on-surface truncate">Erik V.</span>
          <span class="text-label-caps text-[10px] text-primary/70 uppercase tracking-widest">Pro Member</span>
        </div>
      </div>
    </div>
  </aside>
</template>
