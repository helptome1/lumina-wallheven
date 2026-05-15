<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'
import { computed } from 'vue'

const route = useRoute()
const router = useRouter()

const activePath = computed(() => route.path)

function isActive(path: string): boolean {
  return activePath.value === path
}

function navigate(path: string) {
  router.push(path)
}
</script>

<template>
  <aside class="w-sidebar_width h-screen sticky left-0 top-0 z-20 flex flex-col p-6 shadow-xl m-4 rounded-2xl border border-black/5 bg-white/40 backdrop-blur-[40px]">
    <div class="mb-10 px-2">
      <h1 class="text-headline-xl text-primary leading-tight">Lumina</h1>
      <p class="text-body-sm text-on-surface-variant font-medium">4K Creative Gallery</p>
    </div>
    <nav class="flex-1 space-y-2">
      <a
        class="flex items-center gap-3 rounded-xl px-4 py-3 transition-all duration-300"
        :class="isActive('/hot')
          ? 'bg-primary text-on-primary font-semibold border border-primary/20 shadow-lg shadow-primary/20'
          : 'text-on-surface-variant hover:text-on-surface hover:bg-black/5'"
        href="#"
        @click.prevent="navigate('/hot')"
      >
        <span class="material-symbols-outlined" data-icon="language">language</span>
        <span class="text-body-md">Online Wallpapers</span>
      </a>
      <a
        class="flex items-center gap-3 rounded-xl px-4 py-3 transition-all duration-300"
        :class="isActive('/trending')
          ? 'bg-primary text-on-primary font-semibold border border-primary/20 shadow-lg shadow-primary/20'
          : 'text-on-surface-variant hover:text-on-surface hover:bg-black/5'"
        href="#"
        @click.prevent="navigate('/trending')"
      >
        <span class="material-symbols-outlined" data-icon="trending_up">trending_up</span>
        <span class="text-body-md">Trending</span>
      </a>
      <a
        class="flex items-center gap-3 rounded-xl px-4 py-3 transition-all duration-300"
        :class="isActive('/anime')
          ? 'bg-primary text-on-primary font-semibold border border-primary/20 shadow-lg shadow-primary/20'
          : 'text-on-surface-variant hover:text-on-surface hover:bg-black/5'"
        href="#"
        @click.prevent="navigate('/anime')"
      >
        <span class="material-symbols-outlined" data-icon="auto_awesome">auto_awesome</span>
        <span class="text-body-md">Anime</span>
      </a>
      <a
        class="flex items-center gap-3 rounded-xl px-4 py-3 transition-all duration-300"
        :class="isActive('/people')
          ? 'bg-primary text-on-primary font-semibold border border-primary/20 shadow-lg shadow-primary/20'
          : 'text-on-surface-variant hover:text-on-surface hover:bg-black/5'"
        href="#"
        @click.prevent="navigate('/people')"
      >
        <span class="material-symbols-outlined" data-icon="groups">groups</span>
        <span class="text-body-md">People</span>
      </a>
      <a
        class="flex items-center gap-3 rounded-xl px-4 py-3 transition-all duration-300"
        :class="isActive('/collection')
          ? 'bg-primary text-on-primary font-semibold border border-primary/20 shadow-lg shadow-primary/20'
          : 'text-on-surface-variant hover:text-on-surface hover:bg-black/5'"
        href="#"
        @click.prevent="navigate('/collection')"
      >
        <span class="material-symbols-outlined" data-icon="favorite">favorite</span>
        <span class="text-body-md">Favorites</span>
      </a>
      <a
        class="flex items-center gap-3 rounded-xl px-4 py-3 transition-all duration-300"
        :class="isActive('/downloads')
          ? 'bg-primary text-on-primary font-semibold border border-primary/20 shadow-lg shadow-primary/20'
          : 'text-on-surface-variant hover:text-on-surface hover:bg-black/5'"
        href="#"
        @click.prevent="navigate('/downloads')"
      >
        <span class="material-symbols-outlined" data-icon="download">download</span>
        <span class="text-body-md">Downloads</span>
      </a>
    </nav>
    <div class="mt-auto pt-6 border-t border-black/5">
      <a
        class="flex items-center gap-3 rounded-xl px-4 py-3 transition-all mb-4"
        :class="isActive('/about')
          ? 'bg-primary text-on-primary font-bold border border-primary/20 shadow-lg shadow-primary/20'
          : 'text-on-surface-variant hover:text-on-surface hover:bg-black/5'"
        href="#"
        @click.prevent="navigate('/about')"
      >
        <span class="material-symbols-outlined" data-icon="info">info</span>
        <span class="text-body-md">About</span>
      </a>
      <div class="flex items-center gap-3 px-4 py-3 bg-surface-container-low rounded-2xl border border-black/5">
        <div class="w-10 h-10 rounded-full overflow-hidden border-2 border-primary/20">
          <img
            alt="User profile"
            src="https://lh3.googleusercontent.com/aida-public/AB6AXuDbD695TQw9rqMEPG-hAyZ399j2Gb0QS9NmCW_FkouO6ziwiWkxocgmqSJt2_SmwuTJD8mJDw4LbaATG3GfdSdyE5Px0Y4oZPAO_pYeWmgz21reJJW8taCpep_S6kj2NOFJKtLrKMdD34vd7PXBWA2Z9JpNpZvLs1dBQHz3lNINLTfpn1uxe8u1FzJCnv2oISLuLBYwEBlRD0SXGPyoMpBKVUROgjumO-BuqOodL-D6sIBwrSfdO_HqRHWghJl6_8VoKIVhpEWN0-iR"
            class=""
          />
        </div>
        <div class="flex flex-col">
          <span class="text-body-sm font-bold text-on-surface">Erik V.</span>
          <span class="text-label-caps text-[10px] text-primary/70 uppercase tracking-widest">Pro Member</span>
        </div>
      </div>
    </div>
  </aside>
</template>

<style scoped>
aside {
  margin: 1rem;
  width: 280px;
  height: calc(100vh - 2rem);
}
</style>
