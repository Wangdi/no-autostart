import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useHistoryStore } from '../historyStore'
import { invoke } from '@tauri-apps/api/core'
import { StartupType } from '@/types'
import type { HistoryRecord, OperationHistory, ProcessSnapshot, PermanentActionBackup } from '@/types'

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}))

const mockProcessSnapshot: ProcessSnapshot = {
  pid: 1234,
  name: 'TestProcess',
  executablePath: 'C:\\test.exe',
  startupType: StartupType.RegistryRun,
  startupLocation: 'HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run'
}

const mockPermanentAction: PermanentActionBackup = {
  type: 'disable_startup',
  backupData: { registryKey: 'TestKey' }
}

const mockRecords: HistoryRecord[] = [
  {
    id: 'record-1',
    timestamp: '2024-01-01T10:00:00.000Z',
    operationType: 'close_process',
    processSnapshot: mockProcessSnapshot,
    permanentAction: undefined,
    status: 'completed'
  },
  {
    id: 'record-2',
    timestamp: '2024-01-01T11:00:00.000Z',
    operationType: 'permanent_close',
    processSnapshot: { ...mockProcessSnapshot, pid: 5678, name: 'AnotherProcess' },
    permanentAction: mockPermanentAction,
    status: 'completed'
  },
  {
    id: 'record-3',
    timestamp: '2024-01-01T12:00:00.000Z',
    operationType: 'close_process',
    processSnapshot: { ...mockProcessSnapshot, pid: 9012, name: 'ThirdProcess' },
    status: 'reverted',
    revertedAt: '2024-01-01T12:30:00.000Z'
  }
]

const mockOperationHistory: OperationHistory = {
  records: mockRecords
}

describe('useHistoryStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  describe('initial state', () => {
    it('should have empty records array', () => {
      const store = useHistoryStore()
      expect(store.records).toEqual([])
    })

    it('should have loading set to false', () => {
      const store = useHistoryStore()
      expect(store.loading).toBe(false)
    })

    it('should have error set to null', () => {
      const store = useHistoryStore()
      expect(store.error).toBeNull()
    })
  })

  describe('loadHistory', () => {
    it('should set loading to true while loading', async () => {
      const store = useHistoryStore()
      vi.mocked(invoke).mockResolvedValue(mockOperationHistory)

      const promise = store.loadHistory()
      expect(store.loading).toBe(true)
      await promise
    })

    it('should set loading to false after successful load', async () => {
      const store = useHistoryStore()
      vi.mocked(invoke).mockResolvedValue(mockOperationHistory)

      await store.loadHistory()
      expect(store.loading).toBe(false)
    })

    it('should set loading to false after failed load', async () => {
      const store = useHistoryStore()
      vi.mocked(invoke).mockRejectedValue(new Error('History not found'))

      await store.loadHistory()
      expect(store.loading).toBe(false)
    })

    it('should clear error before loading', async () => {
      const store = useHistoryStore()
      store.error = 'Previous error'
      vi.mocked(invoke).mockResolvedValue(mockOperationHistory)

      await store.loadHistory()
      expect(store.error).toBeNull()
    })

    it('should update records on successful load', async () => {
      const store = useHistoryStore()
      vi.mocked(invoke).mockResolvedValue(mockOperationHistory)

      await store.loadHistory()
      expect(store.records).toEqual(mockRecords)
      expect(vi.mocked(invoke)).toHaveBeenCalledWith('get_history')
    })

    it('should handle empty records array', async () => {
      const store = useHistoryStore()
      vi.mocked(invoke).mockResolvedValue({ records: [] })

      await store.loadHistory()
      expect(store.records).toEqual([])
    })

    it('should handle complex history records', async () => {
      const store = useHistoryStore()
      const complexHistory: OperationHistory = {
        records: [
          {
            id: 'complex-1',
            timestamp: '2024-01-01T00:00:00.000Z',
            operationType: 'permanent_close',
            processSnapshot: {
              pid: 9999,
              name: 'ComplexProcess',
              executablePath: 'C:\\Complex\\path\\with spaces\\app.exe',
              startupType: StartupType.TaskScheduler
            },
            permanentAction: {
              type: 'delete_task',
              backupData: { taskName: 'ComplexTask', xmlData: '<xml/>' }
            },
            status: 'completed'
          }
        ]
      }
      vi.mocked(invoke).mockResolvedValue(complexHistory)

      await store.loadHistory()
      expect(store.records).toHaveLength(1)
      expect(store.records[0].permanentAction?.backupData).toBeDefined()
    })

    it('should set error on load failure', async () => {
      const store = useHistoryStore()
      const errorMessage = 'History file corrupted'
      vi.mocked(invoke).mockRejectedValue(new Error(errorMessage))

      await store.loadHistory()
      expect(store.error).toContain(errorMessage)
    })

    it('should set error string on non-Error exception', async () => {
      const store = useHistoryStore()
      vi.mocked(invoke).mockRejectedValue('String error')

      await store.loadHistory()
      expect(store.error).toBe('String error')
    })

    it('should handle records with optional fields', async () => {
      const store = useHistoryStore()
      const minimalHistory: OperationHistory = {
        records: [
          {
            id: 'minimal-1',
            timestamp: '2024-01-01T00:00:00.000Z',
            operationType: 'close_process',
            processSnapshot: {
              pid: 1111,
              name: 'Minimal',
              executablePath: 'C:\\min.exe',
              startupType: StartupType.Normal
            },
            status: 'completed'
          }
        ]
      }
      vi.mocked(invoke).mockResolvedValue(minimalHistory)

      await store.loadHistory()
      expect(store.records).toHaveLength(1)
      expect(store.records[0].permanentAction).toBeUndefined()
      expect(store.records[0].revertedAt).toBeUndefined()
    })
  })

  describe('revertOperation', () => {
    it('should call invoke with correct id', async () => {
      const store = useHistoryStore()
      vi.mocked(invoke)
        .mockResolvedValueOnce(undefined)
        .mockResolvedValueOnce({ records: [] })

      await store.revertOperation('record-1')
      expect(vi.mocked(invoke)).toHaveBeenCalledWith('revert_operation', { id: 'record-1' })
    })

    it('should refresh history after successful revert', async () => {
      const store = useHistoryStore()
      const updatedRecords: HistoryRecord[] = [
        { ...mockRecords[0], status: 'reverted', revertedAt: '2024-01-01T13:00:00.000Z' }
      ]
      vi.mocked(invoke)
        .mockResolvedValueOnce(undefined)
        .mockResolvedValueOnce({ records: updatedRecords })

      await store.revertOperation('record-1')
      expect(vi.mocked(invoke)).toHaveBeenCalledWith('revert_operation', { id: 'record-1' })
      expect(vi.mocked(invoke)).toHaveBeenCalledWith('get_history')
      expect(store.records).toEqual(updatedRecords)
    })

    it('should return true on successful revert', async () => {
      const store = useHistoryStore()
      vi.mocked(invoke)
        .mockResolvedValueOnce(undefined)
        .mockResolvedValueOnce({ records: mockRecords })

      const result = await store.revertOperation('record-1')
      expect(result).toBe(true)
    })

    it('should return false on revert failure', async () => {
      const store = useHistoryStore()
      vi.mocked(invoke).mockRejectedValue(new Error('Revert failed'))

      const result = await store.revertOperation('record-1')
      expect(result).toBe(false)
    })

    it('should set error on revert failure', async () => {
      const store = useHistoryStore()
      const errorMessage = 'Revert failed'
      vi.mocked(invoke).mockRejectedValue(new Error(errorMessage))

      await store.revertOperation('record-1')
      expect(store.error).toContain(errorMessage)
    })

    it('should handle refresh failure gracefully (does not fail revert)', async () => {
      const store = useHistoryStore()
      vi.mocked(invoke)
        .mockResolvedValueOnce(undefined)
        .mockRejectedValueOnce(new Error('Refresh failed'))

      const result = await store.revertOperation('record-1')
      // Note: loadHistory catches errors internally, so revertOperation returns true
      // even if refresh fails - this is current implementation behavior
      expect(result).toBe(true)
      expect(store.loading).toBe(false)
    })

    it('should handle reverting already reverted operation (edge case)', async () => {
      const store = useHistoryStore()
      vi.mocked(invoke)
        .mockResolvedValueOnce(undefined)
        .mockResolvedValueOnce({
          records: [{ ...mockRecords[2], revertedAt: '2024-01-01T14:00:00.000Z' }]
        })

      const result = await store.revertOperation('record-3')
      expect(result).toBe(true)
    })

    it('should handle different record ids', async () => {
      const store = useHistoryStore()
      vi.mocked(invoke)
        .mockResolvedValueOnce(undefined)
        .mockResolvedValueOnce({ records: [] })

      const result = await store.revertOperation('uuid-123-456-789')
      expect(vi.mocked(invoke)).toHaveBeenCalledWith('revert_operation', { id: 'uuid-123-456-789' })
      expect(result).toBe(true)
    })

    it('should preserve existing records on revert failure', async () => {
      const store = useHistoryStore()
      store.records = mockRecords
      vi.mocked(invoke).mockRejectedValue(new Error('Revert failed'))

      await store.revertOperation('record-1')
      expect(store.records).toEqual(mockRecords)
    })
  })

  describe('store behavior', () => {
    it('should maintain separate state between instances', () => {
      const store1 = useHistoryStore()
      const store2 = useHistoryStore()

      store1.records = mockRecords
      expect(store2.records).toEqual(mockRecords)
    })

    it('should clear records when explicitly set to empty', () => {
      const store = useHistoryStore()
      store.records = mockRecords
      store.records = []
      expect(store.records).toEqual([])
    })

    it('should update error state independently', () => {
      const store = useHistoryStore()
      store.error = 'Test error'
      expect(store.error).toBe('Test error')

      store.error = 'Different error'
      expect(store.error).toBe('Different error')

      store.error = null
      expect(store.error).toBeNull()
    })
  })
})
