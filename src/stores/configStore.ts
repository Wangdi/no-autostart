import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { AutoCloseConfig, AutoCloseItem, AppSettings } from '@/types'
import { DEFAULT_CONFIG } from '@/types'

export const useConfigStore = defineStore('config', () => {
  const config = ref<AutoCloseConfig>({ ...DEFAULT_CONFIG })
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function loadConfig() {
    loading.value = true
    error.value = null
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      config.value = await invoke<AutoCloseConfig>('get_config')
    } catch (e) {
      error.value = String(e)
      console.error('Failed to load config:', e)
    } finally {
      loading.value = false
    }
  }

  async function saveConfig(newConfig?: Partial<AutoCloseConfig>) {
    if (newConfig) {
      config.value = { ...config.value, ...newConfig }
    }
    config.value.lastUpdated = new Date().toISOString()

    try {
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('save_config', { config: config.value })
      return true
    } catch (e) {
      error.value = String(e)
      console.error('Failed to save config:', e)
      return false
    }
  }

  async function addToAutoCloseList(item: Omit<AutoCloseItem, 'id' | 'addedAt'>) {
    const newItem: AutoCloseItem = {
      ...item,
      id: crypto.randomUUID(),
      addedAt: new Date().toISOString(),
    }
    config.value.autoCloseList.push(newItem)
    return saveConfig()
  }

  async function removeFromAutoCloseList(id: string) {
    config.value.autoCloseList = config.value.autoCloseList.filter(i => i.id !== id)
    return saveConfig()
  }

  async function updateSettings(settings: Partial<AppSettings>) {
    config.value.settings = { ...config.value.settings, ...settings }
    return saveConfig()
  }

  return {
    config,
    loading,
    error,
    loadConfig,
    saveConfig,
    addToAutoCloseList,
    removeFromAutoCloseList,
    updateSettings,
  }
})
