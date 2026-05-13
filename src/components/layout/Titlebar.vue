<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'
import { Minus, Square, X, Maximize2 } from 'lucide-vue-next'
import { ref, onMounted } from 'vue'

const appWindow = getCurrentWindow()
const isMaximized = ref(false)

onMounted(async () => {
  isMaximized.value = await appWindow.isMaximized()
})

async function minimize() { await appWindow.minimize() }
async function toggleMaximize() {
  await appWindow.toggleMaximize()
  isMaximized.value = await appWindow.isMaximized()
}
async function close() { await appWindow.close() }
</script>

<template>
  <header
    data-tauri-drag-region
    class="titlebar"
  >
    <!-- App name -->
    <div class="titlebar-brand">
      <span class="display-italic text-accent text-[14px] tracking-wide">Gallery</span>
      <div class="w-1 h-1 rounded-full bg-accent/60 ml-2 mt-px" />
    </div>

    <!-- Window controls -->
    <div class="titlebar-controls">
      <button @click="minimize" class="ctrl-btn" aria-label="Minimize">
        <Minus :size="14" />
      </button>
      <button @click="toggleMaximize" class="ctrl-btn" aria-label="Maximize">
        <Maximize2 v-if="!isMaximized" :size="12" />
        <Square v-else :size="12" />
      </button>
      <button @click="close" class="ctrl-btn ctrl-close" aria-label="Close">
        <X :size="14" />
      </button>
    </div>
  </header>
</template>

<style scoped>
.titlebar {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 100;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 14px;
  background: rgba(7, 7, 26, 0.85);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border-bottom: 1px solid var(--border-subtle);
}

.titlebar-brand {
  display: flex;
  align-items: center;
  padding-left: 4px;
}

.titlebar-controls {
  display: flex;
  align-items: center;
  gap: 2px;
  -webkit-app-region: no-drag;
}

.ctrl-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 26px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  transition: all 0.15s ease;
}
.ctrl-btn:hover {
  background: rgba(139, 92, 246, 0.1);
  color: var(--text-secondary);
}
.ctrl-close:hover {
  background: rgba(251, 113, 133, 0.2);
  color: var(--favorite);
}
</style>
