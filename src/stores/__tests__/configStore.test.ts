import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useConfigStore } from '../configStore'
import { invoke } from '@tauri-apps/api/core'
import { DEFAULT_CONFIG, DEFAULT_SETTINGS } from '@/types'
import type { AutoCloseConfig, AutoCloseItem, AppSettings, PermanentAction } from '@/types'

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}))

describe('useConfigStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  afterEach(() => {
    vi.useRealTimers()
  })

  describe('initial state', () => {
    it('should have DEFAULT_CONFIG as initial config', () => {
      const store = useConfigStore()
      expect(store.config.version).toBe(DEFAULT_CONFIG.version)
      expect(store.config.autoCloseList).toEqual([])
      expect(store.config.settings).toEqual(DEFAULT_SETTINGS)
    })

    it('should have loading set to false', () => {
      const store = useConfigStore()
      expect(store.loading).toBe(false)
    })

    it('should have error set to null', () => {
      const store = useConfigStore()
      expect(store.error).toBeNull()
    })

    it('should have config with lastUpdated set', () => {
      const store = useConfigStore()
      expect(store.config.lastUpdated).toBeDefined()
    })
  })

  describe('loadConfig', () => {
    it('should set loading to true while loading', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(DEFAULT_CONFIG)

      const promise = store.loadConfig()
      expect(store.loading).toBe(true)
      await promise
    })

    it('should set loading to false after successful load', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(DEFAULT_CONFIG)

      await store.loadConfig()
      expect(store.loading).toBe(false)
    })

    it('should set loading to false after failed load', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockRejectedValue(new Error('Config not found'))

      await store.loadConfig()
      expect(store.loading).toBe(false)
    })

    it('should clear error before loading', async () => {
      const store = useConfigStore()
      store.error = 'Previous error'
      vi.mocked(invoke).mockResolvedValue(DEFAULT_CONFIG)

      await store.loadConfig()
      expect(store.error).toBeNull()
    })

    it('should update config on successful load', async () => {
      const store = useConfigStore()
      const mockConfig: AutoCloseConfig = {
        version: '2.0.0',
        lastUpdated: '2024-01-01T00:00:00.000Z',
        autoCloseList: [],
        settings: {
          autoRunOnLogin: false,
          autoCloseOnStart: true,
          checkInterval: 5000,
          showNotification: false
        }
      }
      vi.mocked(invoke).mockResolvedValue(mockConfig)

      await store.loadConfig()
      expect(store.config).toEqual(mockConfig)
      expect(vi.mocked(invoke)).toHaveBeenCalledWith('get_config')
    })

    it('should set error on load failure', async () => {
      const store = useConfigStore()
      const errorMessage = 'Config file corrupted'
      vi.mocked(invoke).mockRejectedValue(new Error(errorMessage))

      await store.loadConfig()
      expect(store.error).toContain(errorMessage)
    })

    it('should set error string on non-Error exception', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockRejectedValue('String error')

      await store.loadConfig()
      expect(store.error).toBe('String error')
    })

    it('should handle config with autoCloseList items', async () => {
      const store = useConfigStore()
      const mockItems: AutoCloseItem[] = [
        {
          id: 'test-id-1',
          processName: 'TestProcess',
          executablePath: 'C:\\test.exe',
          addedAt: '2024-01-01T00:00:00.000Z'
        }
      ]
      const mockConfig: AutoCloseConfig = {
        ...DEFAULT_CONFIG,
        autoCloseList: mockItems
      }
      vi.mocked(invoke).mockResolvedValue(mockConfig)

      await store.loadConfig()
      expect(store.config.autoCloseList).toEqual(mockItems)
    })
  })

  describe('saveConfig', () => {
    it('should update lastUpdated on save', async () => {
      const store = useConfigStore()
      vi.useFakeTimers()
      const originalTimestamp = store.config.lastUpdated

      vi.mocked(invoke).mockResolvedValue(undefined)
      vi.advanceTimersByTime(100)
      await store.saveConfig()

      expect(store.config.lastUpdated).not.toBe(originalTimestamp)
      vi.useRealTimers()
    })

    it('should merge new config when provided', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)

      await store.saveConfig({ version: '3.0.0' })
      expect(store.config.version).toBe('3.0.0')
    })

    it('should call invoke with updated config', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)

      await store.saveConfig()
      expect(vi.mocked(invoke)).toHaveBeenCalledWith('save_config', { config: store.config })
    })

    it('should return true on successful save', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)

      const result = await store.saveConfig()
      expect(result).toBe(true)
    })

    it('should return false on save failure', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockRejectedValue(new Error('Permission denied'))

      const result = await store.saveConfig()
      expect(result).toBe(false)
    })

    it('should set error on save failure', async () => {
      const store = useConfigStore()
      const errorMessage = 'Permission denied'
      vi.mocked(invoke).mockRejectedValue(new Error(errorMessage))

      await store.saveConfig()
      expect(store.error).toContain(errorMessage)
    })

    it('should preserve existing config when save fails', async () => {
      const store = useConfigStore()
      const originalConfig = { ...store.config }
      vi.mocked(invoke).mockRejectedValue(new Error('Save failed'))

      await store.saveConfig({ version: '99.0.0' })
      expect(store.config.version).toBe('99.0.0')
    })
  })

  describe('addToAutoCloseList', () => {
    it('should generate a unique id for new item', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)

      await store.addToAutoCloseList({
        processName: 'TestProcess',
        executablePath: 'C:\\test.exe'
      })

      expect(store.config.autoCloseList[0].id).toBeDefined()
      expect(typeof store.config.autoCloseList[0].id).toBe('string')
    })

    it('should add current timestamp', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)

      await store.addToAutoCloseList({
        processName: 'TestProcess',
        executablePath: 'C:\\test.exe'
      })

      const addedAt = store.config.autoCloseList[0].addedAt
      expect(addedAt).toBeDefined()
      expect(new Date(addedAt).getTime()).toBeGreaterThan(0)
      // Verify it's a valid ISO date string
      expect(new Date(addedAt).toISOString()).toBe(addedAt)
    })

    it('should include all provided fields', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)
      store.config.autoCloseList = [] // Ensure clean state
      const permanentAction: PermanentAction = {
        type: 'disable_startup',
        description: 'Disabled startup entry',
        executedAt: '2024-01-01T00:00:00.000Z'
      }

      await store.addToAutoCloseList({
        processName: 'TestProcess',
        executablePath: 'C:\\test.exe',
        permanentAction
      })

      expect(store.config.autoCloseList[0].processName).toBe('TestProcess')
      expect(store.config.autoCloseList[0].executablePath).toBe('C:\\test.exe')
      expect(store.config.autoCloseList[0].permanentAction).toEqual(permanentAction)
    })

    it('should add item to autoCloseList array', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)
      store.config.autoCloseList = [] // Ensure clean state

      await store.addToAutoCloseList({
        processName: 'FirstProcess',
        executablePath: 'C:\\first.exe'
      })

      expect(store.config.autoCloseList).toHaveLength(1)
    })

    it('should append to existing items', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)
      store.config.autoCloseList = [] // Start with clean state

      await store.addToAutoCloseList({
        processName: 'FirstProcess',
        executablePath: 'C:\\first.exe'
      })

      await store.addToAutoCloseList({
        processName: 'SecondProcess',
        executablePath: 'C:\\second.exe'
      })

      expect(store.config.autoCloseList).toHaveLength(2)
      expect(store.config.autoCloseList[0].processName).toBe('FirstProcess')
      expect(store.config.autoCloseList[1].processName).toBe('SecondProcess')
    })

    it('should call saveConfig after adding', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)

      await store.addToAutoCloseList({
        processName: 'TestProcess',
        executablePath: 'C:\\test.exe'
      })

      expect(vi.mocked(invoke)).toHaveBeenCalledWith('save_config', { config: store.config })
    })

    it('should return true on success', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)

      const result = await store.addToAutoCloseList({
        processName: 'TestProcess',
        executablePath: 'C:\\test.exe'
      })

      expect(result).toBe(true)
    })

    it('should return false on save failure', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockRejectedValue(new Error('Save failed'))

      const result = await store.addToAutoCloseList({
        processName: 'TestProcess',
        executablePath: 'C:\\test.exe'
      })

      expect(result).toBe(false)
    })
  })

  describe('removeFromAutoCloseList', () => {
    it('should remove item with matching id', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)

      store.config.autoCloseList = [
        { id: 'id-1', processName: 'First', executablePath: 'C:\\first.exe', addedAt: '2024-01-01' },
        { id: 'id-2', processName: 'Second', executablePath: 'C:\\second.exe', addedAt: '2024-01-02' },
        { id: 'id-3', processName: 'Third', executablePath: 'C:\\third.exe', addedAt: '2024-01-03' }
      ]

      await store.removeFromAutoCloseList('id-2')

      expect(store.config.autoCloseList).toHaveLength(2)
      expect(store.config.autoCloseList.find(i => i.id === 'id-2')).toBeUndefined()
    })

    it('should preserve other items', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)

      // Start with exactly one item
      store.config.autoCloseList = [
        { id: 'id-1', processName: 'First', executablePath: 'C:\\first.exe', addedAt: '2024-01-01' }
      ]

      await store.removeFromAutoCloseList('non-existent-id')

      expect(store.config.autoCloseList).toHaveLength(1)
      expect(store.config.autoCloseList[0].id).toBe('id-1')
    })

    it('should handle empty list', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)
      store.config.autoCloseList = [] // Ensure clean empty state

      const result = await store.removeFromAutoCloseList('any-id')

      expect(result).toBe(true)
      expect(store.config.autoCloseList).toHaveLength(0)
    })

    it('should call saveConfig after removing', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)

      store.config.autoCloseList = [
        { id: 'id-1', processName: 'First', executablePath: 'C:\\first.exe', addedAt: '2024-01-01' }
      ]

      await store.removeFromAutoCloseList('id-1')

      expect(vi.mocked(invoke)).toHaveBeenCalledWith('save_config', { config: store.config })
    })

    it('should return true on success', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)

      store.config.autoCloseList = [
        { id: 'id-1', processName: 'First', executablePath: 'C:\\first.exe', addedAt: '2024-01-01' }
      ]

      const result = await store.removeFromAutoCloseList('id-1')
      expect(result).toBe(true)
    })

    it('should return false on save failure', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockRejectedValue(new Error('Save failed'))

      store.config.autoCloseList = [
        { id: 'id-1', processName: 'First', executablePath: 'C:\\first.exe', addedAt: '2024-01-01' }
      ]

      const result = await store.removeFromAutoCloseList('id-1')
      expect(result).toBe(false)
    })
  })

  describe('updateSettings', () => {
    it('should update single setting', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)

      await store.updateSettings({ checkInterval: 1000 })

      expect(store.config.settings.checkInterval).toBe(1000)
    })

    it('should merge multiple settings', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)

      await store.updateSettings({
        checkInterval: 5000,
        showNotification: false
      })

      expect(store.config.settings.checkInterval).toBe(5000)
      expect(store.config.settings.showNotification).toBe(false)
    })

    it('should preserve existing settings not being updated', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)

      store.config.settings.checkInterval = 3000
      await store.updateSettings({ showNotification: false })

      expect(store.config.settings.checkInterval).toBe(3000)
      expect(store.config.settings.showNotification).toBe(false)
    })

    it('should update autoRunOnLogin', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)

      await store.updateSettings({ autoRunOnLogin: false })

      expect(store.config.settings.autoRunOnLogin).toBe(false)
    })

    it('should update autoCloseOnStart', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)

      await store.updateSettings({ autoCloseOnStart: false })

      expect(store.config.settings.autoCloseOnStart).toBe(false)
    })

    it('should call saveConfig after updating', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)

      await store.updateSettings({ checkInterval: 1000 })

      expect(vi.mocked(invoke)).toHaveBeenCalledWith('save_config', { config: store.config })
    })

    it('should return true on success', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockResolvedValue(undefined)

      const result = await store.updateSettings({ checkInterval: 1000 })
      expect(result).toBe(true)
    })

    it('should return false on save failure', async () => {
      const store = useConfigStore()
      vi.mocked(invoke).mockRejectedValue(new Error('Save failed'))

      const result = await store.updateSettings({ checkInterval: 1000 })
      expect(result).toBe(false)
    })
  })
})
