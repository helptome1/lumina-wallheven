import { invoke, convertFileSrc } from '@tauri-apps/api/core'
import type { SearchParams, SearchResponse } from '@/types/wallhaven'

const IMAGE_CDN = 'https://w.wallhaven.cc/full'

class WallhavenAPI {
  async search(params: SearchParams): Promise<SearchResponse> {
    const queryParams: Record<string, string> = {}
    for (const [key, value] of Object.entries(params)) {
      if (value !== undefined && value !== null && value !== '') {
        queryParams[key] = String(value)
      }
    }

    const apiKey = localStorage.getItem('wallhaven-api-key') || null

    const result = await invoke<SearchResponse>('fetch_wallhaven_api', {
      endpoint: '/search',
      params: queryParams,
      apiKey,
    })

    return result
  }

  getFullImageUrl(path: string): string {
    if (path.startsWith('https://') || path.startsWith('http://')) return path
    return `${IMAGE_CDN}/${path.replace(/^\//, '')}`
  }

  async fetchImage(path: string): Promise<string> {
    const localPath = await invoke<string>('fetch_wallpaper_image', { path })
    return convertFileSrc(localPath)
  }

  async setDesktopWallpaper(path: string): Promise<void> {
    await invoke('set_desktop_wallpaper', { path })
  }
}

export const wallhavenApi = new WallhavenAPI()
