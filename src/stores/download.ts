import { defineStore } from 'pinia'
import { ref } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import type { WallpaperData, DownloadItem } from '@/types/wallhaven'

export const useDownloadStore = defineStore('download', () => {
  const items = ref<DownloadItem[]>(loadFromStorage())
  const activeIds = ref<Set<string>>(new Set())
  let unlistenProgress: UnlistenFn | null = null
  let unlistenError: UnlistenFn | null = null

  function loadFromStorage(): DownloadItem[] {
    try {
      const raw = localStorage.getItem('wallhaven-downloads')
      return raw ? JSON.parse(raw) : []
    } catch {
      return []
    }
  }

  function persist() {
    // Debounced persistence
    const data = items.value.slice(0, 100)
    localStorage.setItem('wallhaven-downloads', JSON.stringify(data))
  }

  async function setupListeners() {
    if (unlistenProgress) return

    unlistenProgress = await listen<DownloadItem>('download-progress', (event) => {
      const payload = event.payload
      const existing = items.value.find(i => i.id === payload.id)
      if (existing) {
        Object.assign(existing, payload)
      } else {
        items.value.unshift(payload)
      }
      if (payload.state === 'done' || payload.state === 'error') {
        activeIds.value.delete(payload.id)
      }
      persist()
    })

    unlistenError = await listen<{ id: string; error: string }>('download-error', (event) => {
      const { id } = event.payload
      const item = items.value.find(i => i.id === id)
      if (item) item.state = 'error'
      activeIds.value.delete(id)
      persist()
    })
  }

  async function startDownload(data: WallpaperData) {
    const ext = data.path.substring(data.path.lastIndexOf('.')) || '.jpg'
    const file_name = `wallhaven-${data.id}${ext}`

    await invoke('download_image', {
      url: data.path,
      id: data.id,
      fileName: file_name,
      resolution: data.resolution,
    })

    items.value.unshift({
      id: data.id,
      url: data.path,
      file_name,
      resolution: data.resolution,
      received_bytes: 0,
      total_bytes: data.file_size,
      speed_bytes: 0,
      state: 'waiting',
      thumbnail: data.thumbs.small,
    })

    activeIds.value.add(data.id)
    persist()
  }

  async function cancelDownload(id: string) {
    await invoke('cancel_download', { id })
  }

  function getActive() {
    return items.value.filter(i => i.state === 'waiting' || i.state === 'downloading')
  }

  function getCompleted() {
    return items.value.filter(i => i.state === 'done')
  }

  return {
    items,
    activeIds,
    setupListeners,
    startDownload,
    cancelDownload,
    getActive,
    getCompleted,
  }
})
