import { invoke } from '@tauri-apps/api/core'
import type { SearchParams, SearchResponse } from '@/types/wallhaven'

const IMAGE_CDN = 'https://w.wallhaven.cc/full'

class WallhavenAPI {
  async search(params: SearchParams): Promise<SearchResponse> {
    // Convert params to a flat Record<string, string>
    const queryParams: Record<string, string> = {}
    for (const [key, value] of Object.entries(params)) {
      if (value !== undefined && value !== null && value !== '') {
        queryParams[key] = String(value)
      }
    }

    // Proxy through Tauri Rust backend to bypass CORS
    const result = await invoke<SearchResponse>('fetch_wallhaven_api', {
      endpoint: '/search',
      params: queryParams,
    })

    return result
  }

  getFullImageUrl(path: string): string {
    return `${IMAGE_CDN}/${path.replace(/^\//, '')}`
  }
}

export const wallhavenApi = new WallhavenAPI()
