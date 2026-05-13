<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'
import { computed } from 'vue'
import { Flame, Ghost, Users, Heart, Download, Info } from 'lucide-vue-next'

const route = useRoute()
const router = useRouter()

interface NavItem {
  path?: string
  label: string
  icon: any
  type?: 'divider' | 'section'
}

const navItems: NavItem[] = [
  { type: 'section', label: '在线壁纸', icon: null },
  { path: '/hot', label: '热门推荐', icon: Flame },
  { path: '/anime', label: '动漫精选', icon: Ghost },
  { path: '/people', label: '人物精选', icon: Users },
  { type: 'divider', label: '', icon: null },
  { type: 'section', label: '我的壁纸', icon: null },
  { path: '/collection', label: '我的收藏', icon: Heart },
  { path: '/downloads', label: '下载中心', icon: Download },
  { type: 'divider', label: '', icon: null },
  { type: 'section', label: '其他', icon: null },
  { path: '/about', label: '关于', icon: Info },
]

const activePath = computed(() => route.path)
</script>

<template>
  <aside class="sidebar">
    <!-- Brand -->
    <div class="sidebar-brand">
      <p class="display text-[24px] leading-none text-accent tracking-tight">Gallery</p>
      <p class="text-[10px] text-subtle tracking-[0.2em] uppercase mt-1">Wallpaper</p>
    </div>

    <!-- Nav -->
    <nav class="sidebar-nav">
      <template v-for="(item, index) in navItems" :key="index">
        <p v-if="item.type === 'section'" class="nav-section">{{ item.label }}</p>
        <div v-else-if="item.type === 'divider'" class="divider my-1.5" />
        <button
          v-else
          @click="router.push(item.path!)"
          class="nav-item group"
          :class="{ active: activePath === item.path }"
        >
          <component :is="item.icon" :size="17" class="nav-icon" />
          <span class="nav-label">{{ item.label }}</span>
          <span v-if="activePath === item.path" class="nav-indicator" />
        </button>
      </template>
    </nav>

    <!-- Footer -->
    <div class="sidebar-footer">
      <p class="text-[10px] text-subtle leading-relaxed">
        Images from
        <span class="text-accent/60">wallhaven.cc</span>
      </p>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 210px;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  padding: 20px 14px;
  background: rgba(12, 12, 36, 0.5);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border-right: 1px solid var(--border-subtle);
}

.sidebar-brand {
  padding: 0 8px 24px;
}

.nav-section {
  padding: 8px 10px 4px;
  font-size: 0.65rem;
  font-weight: 600;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--text-tertiary);
}

.sidebar-nav {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.nav-item {
  position: relative;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 9px 12px;
  border: none;
  border-radius: var(--radius-md);
  background: transparent;
  color: var(--text-secondary);
  font-family: var(--font-body);
  font-size: 0.8125rem;
  cursor: pointer;
  transition: all 0.2s ease;
  text-align: left;
  overflow: hidden;
}

.nav-item:hover {
  color: var(--text-primary);
  background: rgba(139, 92, 246, 0.08);
}

.nav-item.active {
  color: var(--accent);
  background: var(--accent-glow);
  box-shadow: 0 0 20px var(--accent-glow);
}

.nav-indicator {
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 2px;
  height: 18px;
  border-radius: 1px;
  background: linear-gradient(180deg, var(--accent-soft), var(--accent-2-soft));
  animation: fade-in 0.3s ease;
  box-shadow: 0 0 8px var(--accent-glow-strong);
}

.nav-icon {
  flex-shrink: 0;
  opacity: 0.5;
  transition: opacity 0.2s ease;
}
.nav-item:hover .nav-icon {
  opacity: 0.75;
}
.nav-item.active .nav-icon {
  opacity: 1;
}

.nav-label {
  font-weight: 400;
}
.nav-item.active .nav-label {
  font-weight: 500;
}

.sidebar-footer {
  margin-top: auto;
  padding: 16px 8px 0;
}
</style>
