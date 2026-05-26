import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { HistoryRecord, OperationHistory } from '@/types'

export const useHistoryStore = defineStore('history', () => {
  const records = ref<HistoryRecord[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function loadHistory() {
    loading.value = true
    error.value = null
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      const history = await invoke<OperationHistory>('get_history')
      records.value = history.records
    } catch (e) {
      error.value = String(e)
      console.error('Failed to load history:', e)
    } finally {
      loading.value = false
    }
  }

  async function revertOperation(id: string) {
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('revert_operation', { id })
      await loadHistory()
      return true
    } catch (e) {
      error.value = String(e)
      console.error('Failed to revert operation:', e)
      return false
    }
  }

  return {
    records,
    loading,
    error,
    loadHistory,
    revertOperation,
  }
})
