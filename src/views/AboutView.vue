<script setup lang="ts">
</script>

<template>
  <div class="flex flex-col h-full">
    <div class="px-6 py-4">
      <h2 class="display-italic text-accent text-2xl">关于</h2>
    </div>

    <div class="flex-1 flex items-center justify-center px-6 pb-8">
      <div class="glass-card p-8 rounded-3xl max-w-sm w-full text-center">
        <!-- Logo -->
        <div class="w-16 h-16 rounded-2xl bg-gradient-to-br from-violet-500 to-teal-400 flex items-center justify-center mx-auto mb-6 shadow-[0_0_48px_rgba(139,92,246,0.25)]">
          <span class="display text-2xl text-white">G</span>
        </div>

        <h1 class="display text-xl text-white/90 mb-2">Gallery</h1>
        <p class="text-sm text-muted leading-relaxed mb-6">
          基于 wallhaven.cc 的桌面壁纸浏览器，
          <br />
          使用 Tauri + Vue 3 构建。
        </p>

        <div class="space-y-2 text-xs">
          <div class="flex justify-between py-1.5 border-b border-white/[0.04]">
            <span class="text-subtle">版本</span>
            <span class="text-muted">1.0.0</span>
          </div>
          <div class="flex justify-between py-1.5 border-b border-white/[0.04]">
            <span class="text-subtle">技术栈</span>
            <span class="text-muted">Tauri + Vue 3 + Tailwind</span>
          </div>
          <div class="flex justify-between py-1.5">
            <span class="text-subtle">数据来源</span>
            <span class="text-accent/60">wallhaven.cc</span>
          </div>
        </div>

        <p class="text-[11px] text-subtle mt-6">
          wallhaven.cc 所有壁纸版权归原作者所有
        </p>
      </div>
    </div>
  </div>
</template>
