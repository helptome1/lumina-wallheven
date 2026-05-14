import { defineStore } from 'pinia'
import { reactive } from 'vue'
import type { SearchParams } from '@/types/wallhaven'

export const useFilterStore = defineStore('filter', () => {
  const params = reactive<SearchParams>({
    sorting: 'toplist',
    purity: '100',
    atleast: '3840x2160',
    q: '',
    page: 1,
  })

  function setPurity(purity: string) {
    params.purity = purity
  }

  function setSorting(sorting: string) {
    params.sorting = sorting
  }

  function setAtleast(atleast: string) {
    params.atleast = atleast
  }

  function setQuery(q: string) {
    params.q = q
  }

  function getSnapshot(): SearchParams {
    return { ...params }
  }

  return { params, setPurity, setSorting, setAtleast, setQuery, getSnapshot }
})
