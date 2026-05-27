import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useProcessStore } from '../processStore'
import { invoke } from '@tauri-apps/api/core'
import { StartupType, RiskLevel, RecommendedAction } from '@/types'
import type { ProcessInfo, ProcessFilter } from '@/types'

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}))

const mockProcesses: ProcessInfo[] = [
  {
    pid: 1234,
    name: 'TestProcess',
    executablePath: 'C:\\Program Files\\TestCompany\\testapp.exe',
    publisher: 'Test Publisher',
    cpuUsage: 5.5,
    memoryUsage: 1024000,
    runningTime: 3600,
    startupType: StartupType.RegistryRun,
    startupLocation: 'HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run',
    localDescription: 'A test process',
    isKnownProcess: true,
    riskLevel: RiskLevel.Safe,
    performanceImpact: 'Low',
    recommendation: 'Safe to close',
    canClose: true,
    recommendedAction: RecommendedAction.CanClose
  },
  {
    pid: 5678,
    name: 'AnotherProcess',
    executablePath: 'C:\\Users\\User\\AppData\\Roaming\\suspicious.exe',
    publisher: 'Unknown',
    cpuUsage: 25.0,
    memoryUsage: 5242880,
    runningTime: 7200,
    startupType: StartupType.StartupFolder,
    startupLocation: 'C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup',
    localDescription: 'Unknown process',
    isKnownProcess: false,
    riskLevel: RiskLevel.Warning,
    performanceImpact: 'High',
    recommendation: 'Should close',
    canClose: false,
    recommendedAction: RecommendedAction.ShouldClose
  },
  {
    pid: 9012,
    name: 'SystemService',
    executablePath: 'C:\\Windows\\System32\\service.exe',
    publisher: 'Microsoft',
    cpuUsage: 1.0,
    memoryUsage: 512000,
    runningTime: 86400,
    startupType: StartupType.WindowsService,
    startupLocation: 'HKLM\\SYSTEM\\CurrentControlSet\\Services',
    localDescription: 'System service',
    isKnownProcess: true,
    riskLevel: RiskLevel.Safe,
    performanceImpact: 'Low',
    recommendation: 'Keep running',
    canClose: true,
    recommendedAction: RecommendedAction.KeepRunning
  }
]

describe('useProcessStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  describe('initial state', () => {
    it('should have empty processes array', () => {
      const store = useProcessStore()
      expect(store.processes).toEqual([])
    })

    it('should have loading set to false', () => {
      const store = useProcessStore()
      expect(store.loading).toBe(false)
    })

    it('should have error set to null', () => {
      const store = useProcessStore()
      expect(store.error).toBeNull()
    })

    it('should have selectedPid set to null', () => {
      const store = useProcessStore()
      expect(store.selectedPid).toBeNull()
    })

    it('should have empty filter object', () => {
      const store = useProcessStore()
      expect(store.filter).toEqual({})
    })
  })

  describe('filteredProcesses computed', () => {
    it('should return all processes when filter is empty', () => {
      const store = useProcessStore()
      store.processes = mockProcesses
      expect(store.filteredProcesses).toHaveLength(3)
      expect(store.filteredProcesses).toEqual(mockProcesses)
    })

    it('should filter by search term matching name', () => {
      const store = useProcessStore()
      store.processes = mockProcesses
      store.filter = { search: 'TestProcess' }
      expect(store.filteredProcesses).toHaveLength(1)
      expect(store.filteredProcesses[0].name).toBe('TestProcess')
    })

    it('should filter by search term matching executable path (case insensitive)', () => {
      const store = useProcessStore()
      store.processes = mockProcesses
      store.filter = { search: 'suspicious.exe' }
      expect(store.filteredProcesses).toHaveLength(1)
      expect(store.filteredProcesses[0].name).toBe('AnotherProcess')
    })

    it('should filter by search term (case insensitive)', () => {
      const store = useProcessStore()
      store.processes = mockProcesses
      store.filter = { search: 'SYSTEM' }
      expect(store.filteredProcesses).toHaveLength(1)
      expect(store.filteredProcesses[0].name).toBe('SystemService')
    })

    it('should return empty array when search matches nothing', () => {
      const store = useProcessStore()
      store.processes = mockProcesses
      store.filter = { search: 'NonExistent' }
      expect(store.filteredProcesses).toHaveLength(0)
    })

    it('should filter by startup types', () => {
      const store = useProcessStore()
      store.processes = mockProcesses
      store.filter = { startupTypes: [StartupType.RegistryRun] }
      expect(store.filteredProcesses).toHaveLength(1)
      expect(store.filteredProcesses[0].startupType).toBe(StartupType.RegistryRun)
    })

    it('should filter by multiple startup types', () => {
      const store = useProcessStore()
      store.processes = mockProcesses
      store.filter = { startupTypes: [StartupType.RegistryRun, StartupType.WindowsService] }
      expect(store.filteredProcesses).toHaveLength(2)
    })

    it('should filter by risk levels', () => {
      const store = useProcessStore()
      store.processes = mockProcesses
      store.filter = { riskLevels: [RiskLevel.Warning] }
      expect(store.filteredProcesses).toHaveLength(1)
      expect(store.filteredProcesses[0].riskLevel).toBe(RiskLevel.Warning)
    })

    it('should filter by multiple risk levels', () => {
      const store = useProcessStore()
      store.processes = mockProcesses
      store.filter = { riskLevels: [RiskLevel.Safe, RiskLevel.Warning] }
      expect(store.filteredProcesses).toHaveLength(3)
    })

    it('should filter by canCloseOnly', () => {
      const store = useProcessStore()
      store.processes = mockProcesses
      store.filter = { canCloseOnly: true }
      expect(store.filteredProcesses).toHaveLength(2)
      expect(store.filteredProcesses.every(p => p.canClose)).toBe(true)
    })

    it('should combine multiple filters', () => {
      const store = useProcessStore()
      store.processes = mockProcesses
      store.filter = {
        search: 'service',
        startupTypes: [StartupType.WindowsService],
        canCloseOnly: true
      }
      expect(store.filteredProcesses).toHaveLength(1)
      expect(store.filteredProcesses[0].name).toBe('SystemService')
    })

    it('should return empty when combined filters exclude all', () => {
      const store = useProcessStore()
      store.processes = mockProcesses
      store.filter = {
        search: 'TestProcess',
        riskLevels: [RiskLevel.Warning]
      }
      expect(store.filteredProcesses).toHaveLength(0)
    })
  })

  describe('selectedProcess computed', () => {
    it('should return null when selectedPid is null', () => {
      const store = useProcessStore()
      store.processes = mockProcesses
      store.selectedPid = null
      expect(store.selectedProcess).toBeUndefined()
    })

    it('should return correct process when selectedPid matches', () => {
      const store = useProcessStore()
      store.processes = mockProcesses
      store.selectedPid = 5678
      expect(store.selectedProcess).toEqual(mockProcesses[1])
    })

    it('should return undefined when selectedPid does not exist', () => {
      const store = useProcessStore()
      store.processes = mockProcesses
      store.selectedPid = 9999
      expect(store.selectedProcess).toBeUndefined()
    })
  })

  describe('fetchProcesses', () => {
    it('should set loading to true while fetching', async () => {
      const store = useProcessStore()
      vi.mocked(invoke).mockResolvedValue(mockProcesses)

      const promise = store.fetchProcesses()
      expect(store.loading).toBe(true)
      await promise
    })

    it('should set loading to false after successful fetch', async () => {
      const store = useProcessStore()
      vi.mocked(invoke).mockResolvedValue(mockProcesses)

      await store.fetchProcesses()
      expect(store.loading).toBe(false)
    })

    it('should set loading to false after failed fetch', async () => {
      const store = useProcessStore()
      vi.mocked(invoke).mockRejectedValue(new Error('Network error'))

      await store.fetchProcesses()
      expect(store.loading).toBe(false)
    })

    it('should clear error before fetching', async () => {
      const store = useProcessStore()
      store.error = 'Previous error'
      vi.mocked(invoke).mockResolvedValue(mockProcesses)

      await store.fetchProcesses()
      expect(store.error).toBeNull()
    })

    it('should update processes on successful fetch', async () => {
      const store = useProcessStore()
      vi.mocked(invoke).mockResolvedValue(mockProcesses)

      await store.fetchProcesses()
      expect(store.processes).toEqual(mockProcesses)
      expect(vi.mocked(invoke)).toHaveBeenCalledWith('get_all_processes')
    })

    it('should handle empty processes array', async () => {
      const store = useProcessStore()
      vi.mocked(invoke).mockResolvedValue([])

      await store.fetchProcesses()
      expect(store.processes).toEqual([])
    })

    it('should set error on fetch failure', async () => {
      const store = useProcessStore()
      const errorMessage = 'Network error'
      vi.mocked(invoke).mockRejectedValue(new Error(errorMessage))

      await store.fetchProcesses()
      expect(store.error).toContain(errorMessage)
    })

    it('should set error string on non-Error exception', async () => {
      const store = useProcessStore()
      vi.mocked(invoke).mockRejectedValue('String error')

      await store.fetchProcesses()
      expect(store.error).toBe('String error')
    })
  })

  describe('closeProcess', () => {
    it('should call invoke with correct arguments', async () => {
      const store = useProcessStore()
      vi.mocked(invoke).mockResolvedValue(undefined)
      store.processes = mockProcesses

      await store.closeProcess(1234)
      expect(vi.mocked(invoke)).toHaveBeenCalledWith('close_process', { pid: 1234 })
    })

    it('should refresh processes after successful close', async () => {
      const store = useProcessStore()
      vi.mocked(invoke)
        .mockResolvedValueOnce(undefined)
        .mockResolvedValueOnce([])

      await store.closeProcess(1234)
      expect(vi.mocked(invoke)).toHaveBeenCalledWith('close_process', { pid: 1234 })
      expect(vi.mocked(invoke)).toHaveBeenCalledWith('get_all_processes')
    })

    it('should return true on successful close', async () => {
      const store = useProcessStore()
      vi.mocked(invoke)
        .mockResolvedValueOnce(undefined)
        .mockResolvedValueOnce([])

      const result = await store.closeProcess(1234)
      expect(result).toBe(true)
    })

    it('should return false on close failure', async () => {
      const store = useProcessStore()
      vi.mocked(invoke).mockRejectedValue(new Error('Access denied'))

      const result = await store.closeProcess(1234)
      expect(result).toBe(false)
    })

    it('should set error on close failure', async () => {
      const store = useProcessStore()
      const errorMessage = 'Access denied'
      vi.mocked(invoke).mockRejectedValue(new Error(errorMessage))

      await store.closeProcess(1234)
      expect(store.error).toContain(errorMessage)
    })
  })

  describe('setFilter', () => {
    it('should update filter with single property', () => {
      const store = useProcessStore()
      store.setFilter({ search: 'test' })
      expect(store.filter).toEqual({ search: 'test' })
    })

    it('should merge filter with existing properties', () => {
      const store = useProcessStore()
      store.filter = { search: 'test' }
      store.setFilter({ canCloseOnly: true })
      expect(store.filter).toEqual({ search: 'test', canCloseOnly: true })
    })

    it('should override existing properties', () => {
      const store = useProcessStore()
      store.filter = { search: 'old' }
      store.setFilter({ search: 'new' })
      expect(store.filter.search).toBe('new')
    })

    it('should handle complex filter updates', () => {
      const store = useProcessStore()
      const startupTypes: StartupType[] = [StartupType.RegistryRun, StartupType.StartupFolder]
      const riskLevels: RiskLevel[] = [RiskLevel.Safe]

      store.setFilter({
        search: 'test',
        startupTypes,
        riskLevels,
        canCloseOnly: true
      })

      expect(store.filter).toEqual({
        search: 'test',
        startupTypes,
        riskLevels,
        canCloseOnly: true
      })
    })

    it('should handle empty filter update', () => {
      const store = useProcessStore()
      store.filter = { search: 'test' }
      store.setFilter({})
      expect(store.filter).toEqual({ search: 'test' })
    })
  })

  describe('selectProcess', () => {
    it('should update selectedPid', () => {
      const store = useProcessStore()
      store.selectProcess(1234)
      expect(store.selectedPid).toBe(1234)
    })

    it('should allow deselecting by passing null', () => {
      const store = useProcessStore()
      store.selectedPid = 1234
      store.selectProcess(null)
      expect(store.selectedPid).toBeNull()
    })

    it('should update selectedProcess computed', () => {
      const store = useProcessStore()
      store.processes = mockProcesses
      store.selectProcess(5678)
      expect(store.selectedProcess).toEqual(mockProcesses[1])
    })

    it('should allow selecting different processes', () => {
      const store = useProcessStore()
      store.processes = mockProcesses

      store.selectProcess(1234)
      expect(store.selectedPid).toBe(1234)

      store.selectProcess(9012)
      expect(store.selectedPid).toBe(9012)
    })
  })
})
