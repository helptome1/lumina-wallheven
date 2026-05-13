<script setup lang="ts">
import { reactive } from 'vue'
import { Search } from 'lucide-vue-next'
import type { SearchParams } from '@/types/wallhaven'

const emit = defineEmits<{ search: [params: SearchParams] }>()

const params = reactive<SearchParams>({
  sorting: 'toplist',
  purity: '100',
  atleast: '',
  ratios: '',
  q: '',
  page: 1,
})

const purityOptions = [
  { label: 'SFW', value: '100' },
  { label: 'Sketchy', value: '010' },
]

const sortingOptions = [
  { label: '最热', value: 'toplist' },
  { label: '热门', value: 'hot' },
  { label: '最新', value: 'date_added' },
  { label: '最多浏览', value: 'views' },
  { label: '最多收藏', value: 'favorites' },
  { label: '随机', value: 'random' },
]

const resolutionOptions = [
  { label: '全部分辨率', value: '' },
  { label: '1080p+', value: '1920x1080' },
  { label: '2K+', value: '2560x1440' },
  { label: '4K+', value: '3840x2160' },
  { label: '8K+', value: '7680x4320' },
]

const ratioOptions = [
  { label: '全部比例', value: '' },
  { label: '16:9 宽屏', value: '16x9' },
  { label: '16:10', value: '16x10' },
  { label: '21:9 超宽', value: '21x9' },
  { label: '9:16 竖屏', value: '9x16' },
  { label: '1:1 方形', value: '1x1' },
]

function doSearch() {
  params.page = 1
  emit('search', { ...params })
}

function onPurityChange(value: string) {
  params.purity = value
  doSearch()
}
</script>

<template>
  <div class="search-bar flex items-center gap-3 px-6 py-3.5">
    <!-- Purity selector -->
    <div class="flex bg-white/[0.03] rounded-xl p-1 gap-0.5 border border-white/[0.04]">
      <button
        v-for="opt in purityOptions"
        :key="opt.value"
        @click="onPurityChange(opt.value)"
        class="purity-btn"
        :class="{ active: params.purity === opt.value }"
      >
        {{ opt.label }}
      </button>
    </div>

    <!-- Divider -->
    <div class="w-px h-6 bg-white/[0.06]" />

    <!-- Sorting -->
    <div class="relative">
      <select v-model="params.sorting" @change="doSearch" class="glass-select text-xs">
        <option v-for="opt in sortingOptions" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
      </select>
    </div>

    <!-- Resolution -->
    <div class="relative">
      <select v-model="params.atleast" @change="doSearch" class="glass-select text-xs">
        <option v-for="opt in resolutionOptions" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
      </select>
    </div>

    <!-- Ratio -->
    <div class="relative">
      <select v-model="params.ratios" @change="doSearch" class="glass-select text-xs">
        <option v-for="opt in ratioOptions" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
      </select>
    </div>

    <!-- Spacer -->
    <div class="flex-1" />

    <!-- Keyword search -->
    <div class="relative">
      <Search :size="14" class="absolute left-3.5 top-1/2 -translate-y-1/2 text-white/15 pointer-events-none" />
      <input
        v-model="params.q"
        type="text"
        placeholder="搜索壁纸…"
        class="glass-input pl-9 pr-4 py-2 text-xs w-56"
        @keyup.enter="doSearch"
      />
    </div>
  </div>
</template>

<style scoped>
.search-bar {
  background: rgba(12, 12, 36, 0.5);
  backdrop-filter: blur(16px);
  -webkit-backdrop-filter: blur(16px);
  border-bottom: 1px solid var(--border-subtle);
}

.purity-btn {
  padding: 5px 14px;
  font-size: 0.75rem;
  font-weight: 500;
  letter-spacing: 0.02em;
  border: none;
  border-radius: 10px;
  background: transparent;
  color: rgba(255, 255, 255, 0.25);
  cursor: pointer;
  transition: all 0.3s ease;
}
.purity-btn:hover {
  color: rgba(255, 255, 255, 0.5);
}
.purity-btn.active {
  color: var(--accent);
  background: var(--accent-glow);
  box-shadow: 0 0 16px var(--accent-glow);
}

.glass-select {
  appearance: none;
  background-color: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(167, 139, 250, 0.1);
  border-radius: 0.75rem;
  padding: 0.4rem 28px 0.4rem 0.75rem;
  color: rgba(255, 255, 255, 0.45);
  outline: none;
  cursor: pointer;
  font-size: 0.75rem;
  font-family: var(--font-body);
  transition: all 0.2s ease;
  background-image: url("data:image/svg+xml,%3Csvg width='8' height='5' viewBox='0 0 8 5' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M1 1l3 3 3-3' stroke='%23A78BFA44' stroke-width='1.2' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 10px center;
}
.glass-select:focus {
  border-color: rgba(167, 139, 250, 0.4);
  color: rgba(255, 255, 255, 0.75);
  background-color: rgba(255, 255, 255, 0.05);
}

.glass-input {
  background-color: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(167, 139, 250, 0.1);
  border-radius: 0.75rem;
  color: rgba(255, 255, 255, 0.55);
  outline: none;
  font-size: 0.75rem;
  font-family: var(--font-body);
  transition: all 0.2s ease;
}
.glass-input:focus {
  border-color: rgba(167, 139, 250, 0.4);
  background-color: rgba(255, 255, 255, 0.05);
  box-shadow: 0 0 20px var(--accent-glow);
}
.glass-input::placeholder {
  color: rgba(255, 255, 255, 0.1);
}
</style>
