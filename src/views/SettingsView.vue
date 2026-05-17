<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useDownloadStore } from '@/stores/download'

const downloadStore = useDownloadStore()

const apiKey = ref('')
const savedKey = ref('')
const showKey = ref(false)
const saved = ref(false)

const API_KEY_STORAGE = 'wallhaven-api-key'

onMounted(() => {
  savedKey.value = localStorage.getItem(API_KEY_STORAGE) || ''
  apiKey.value = savedKey.value
})

function saveApiKey() {
  const key = apiKey.value.trim()
  if (key) {
    localStorage.setItem(API_KEY_STORAGE, key)
    savedKey.value = key
  } else {
    localStorage.removeItem(API_KEY_STORAGE)
    savedKey.value = ''
  }
  window.dispatchEvent(new CustomEvent('api-key-changed'))
  saved.value = true
  setTimeout(() => { saved.value = false }, 2000)
}

function maskKey(key: string): string {
  if (key.length <= 4) return key
  return key.slice(0, 4) + '•'.repeat(Math.min(key.length - 4, 20))
}

async function changeDownloadDir() {
  await downloadStore.chooseDownloadDir()
}
</script>

<template>
  <div class="flex flex-col h-full p-10 overflow-y-auto custom-scrollbar">
    <h1 class="text-headline-lg text-on-surface mb-10">设置</h1>

    <div class="max-w-2xl space-y-8">
      <!-- API Key -->
      <section class="detail-info-card rounded-2xl p-6">
        <h2 class="font-headline-md text-on-surface mb-1">Wallhaven API Key</h2>
        <p class="text-body-sm text-modal-secondary mb-4">
          用于访问 NSFW 内容及个性化功能。在
          <a href="https://wallhaven.cc/settings/account" target="_blank" class="text-primary underline">wallhaven.cc/settings/account</a>
          获取。
        </p>
        <div class="flex items-center gap-3">
          <div class="relative flex-1 max-w-md">
            <input
              v-model="apiKey"
              :type="showKey ? 'text' : 'password'"
              class="w-full bg-white/60 border border-black/5 rounded-xl py-2.5 px-4 text-body-sm focus:outline-none focus:ring-2 focus:ring-primary/40 transition-all"
              placeholder="输入 API Key…"
            />
            <button
              @click="showKey = !showKey"
              class="absolute right-3 top-1/2 -translate-y-1/2 text-on-surface-variant hover:text-primary transition-colors"
            >
              <span class="material-symbols-outlined text-[20px]">
                {{ showKey ? 'visibility_off' : 'visibility' }}
              </span>
            </button>
          </div>
          <button
            @click="saveApiKey"
            class="px-5 py-2.5 bg-primary text-on-primary rounded-xl font-headline-md hover:brightness-110 transition-all"
          >
            {{ saved ? '已保存' : '保存' }}
          </button>
        </div>
        <p v-if="savedKey" class="text-body-sm text-modal-secondary mt-3">
          当前 Key：{{ maskKey(savedKey) }}
        </p>
      </section>

      <!-- Download Directory -->
      <section class="detail-info-card rounded-2xl p-6">
        <h2 class="font-headline-md text-on-surface mb-1">壁纸下载地址</h2>
        <p class="text-body-sm text-modal-secondary mb-4">下载的壁纸将保存到此目录。</p>
        <div class="flex items-center gap-3">
          <div class="flex-1 max-w-md bg-white/40 border border-black/5 rounded-xl py-2.5 px-4 text-body-sm text-on-surface truncate">
            {{ downloadStore.downloadDir || '未设置' }}
          </div>
          <button
            @click="changeDownloadDir"
            :disabled="downloadStore.selectingDir"
            class="px-5 py-2.5 detail-info-card rounded-xl font-headline-md hover:bg-white/50 transition-all"
          >
            {{ downloadStore.selectingDir ? '选择中…' : '更改' }}
          </button>
        </div>
      </section>
    </div>
  </div>
</template>
