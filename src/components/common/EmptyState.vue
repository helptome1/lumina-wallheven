<script setup lang="ts">
import { ImageOff } from 'lucide-vue-next'

defineProps<{
  title: string
  description?: string
}>()
</script>

<template>
  <div class="flex flex-col items-center justify-center h-full gap-5 py-20">
    <div class="w-16 h-16 rounded-2xl bg-white/[0.02] border border-white/[0.04] flex items-center justify-center">
      <ImageOff :size="28" class="text-white/10" />
    </div>
    <div class="text-center">
      <h3 class="text-base font-medium text-white/20">{{ title }}</h3>
      <p v-if="description" class="text-sm text-white/10 mt-1.5">{{ description }}</p>
    </div>
  </div>
</template>
