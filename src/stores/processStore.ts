import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { ProcessInfo, ProcessFilter } from '@/types'

export const useProcessStore = defineStore('process', () => {
  const processes = ref<ProcessInfo[]>([])
  const filter = ref<ProcessFilter>({})
  const loading = ref(false)
  const error = ref<string | null>(null)
  const selectedPid = ref<number | null>(null)

  const filteredProcesses = computed(() => {
    let result = processes.value

    if (filter.value.search) {
      const search = filter.value.search.toLowerCase()
      result = result.filter(p =>
        p.name.toLowerCase().includes(search) ||
        p.executablePath.toLowerCase().includes(search)
      )
    }

    if (filter.value.startupTypes?.length) {
      result = result.filter(p =>
        filter.value.startupTypes!.includes(p.startupType)
      )
    }

    if (filter.value.riskLevels?.length) {
      result = result.filter(p =>
        filter.value.riskLevels!.includes(p.riskLevel)
      )
    }

    if (filter.value.canCloseOnly) {
      result = result.filter(p => p.canClose)
    }

    return result
  })

  const selectedProcess = computed(() =>
    processes.value.find(p => p.pid === selectedPid.value)
  )

  async function fetchProcesses() {
    loading.value = true
    error.value = null
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      processes.value = await invoke<ProcessInfo[]>('get_all_processes')
    } catch (e) {
      error.value = String(e)
      console.error('Failed to fetch processes:', e)
    } finally {
      loading.value = false
    }
  }

  async function closeProcess(pid: number) {
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('close_process', { pid })
      await fetchProcesses()
      return true
    } catch (e) {
      error.value = String(e)
      console.error('Failed to close process:', e)
      return false
    }
  }

  function setFilter(newFilter: Partial<ProcessFilter>) {
    filter.value = { ...filter.value, ...newFilter }
  }

  function selectProcess(pid: number | null) {
    selectedPid.value = pid
  }

  return {
    processes,
    filter,
    loading,
    error,
    selectedPid,
    filteredProcesses,
    selectedProcess,
    fetchProcesses,
    closeProcess,
    setFilter,
    selectProcess,
  }
})
