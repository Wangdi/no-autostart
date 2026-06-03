import { describe, it, expect } from 'vitest'
import {
  StartupType,
  RiskLevel,
  RecommendedAction,
  STARTUP_TYPE_LABELS,
  RISK_LEVEL_LABELS,
  RISK_LEVEL_COLORS
} from '../process'
import type { ProcessInfo, ProcessFilter } from '../process'

describe('StartupType Enum', () => {
  it('should have all expected enum values', () => {
    expect(StartupType.Unknown).toBe('unknown')
    expect(StartupType.RegistryRun).toBe('registry_run')
    expect(StartupType.RegistryRunOnce).toBe('registry_run_once')
    expect(StartupType.TaskScheduler).toBe('task_scheduler')
    expect(StartupType.WindowsService).toBe('windows_service')
    expect(StartupType.StartupFolder).toBe('startup_folder')
    expect(StartupType.Normal).toBe('normal')
  })

  it('should have exactly 7 values', () => {
    const values = Object.values(StartupType)
    expect(values).toHaveLength(7)
  })

  it('should only contain string values', () => {
    const values = Object.values(StartupType)
    values.forEach((value) => {
      expect(typeof value).toBe('string')
    })
  })

  it('should be usable as type guard', () => {
    function isStartupType(value: string): value is StartupType {
      return Object.values(StartupType).includes(value as StartupType)
    }

    expect(isStartupType('registry_run')).toBe(true)
    expect(isStartupType('normal')).toBe(true)
    expect(isStartupType('invalid')).toBe(false)
    expect(isStartupType('')).toBe(false)
  })
})

describe('RiskLevel Enum', () => {
  it('should have all expected enum values', () => {
    expect(RiskLevel.Safe).toBe('safe')
    expect(RiskLevel.Low).toBe('low')
    expect(RiskLevel.Caution).toBe('caution')
    expect(RiskLevel.Dangerous).toBe('dangerous')
    expect(RiskLevel.Warning).toBe('warning')
    expect(RiskLevel.Unknown).toBe('unknown')
  })

  it('should have exactly 6 values', () => {
    const values = Object.values(RiskLevel)
    expect(values).toHaveLength(6)
  })

  it('should only contain string values', () => {
    const values = Object.values(RiskLevel)
    values.forEach((value) => {
      expect(typeof value).toBe('string')
    })
  })

  it('should be usable as type guard', () => {
    function isRiskLevel(value: string): value is RiskLevel {
      return Object.values(RiskLevel).includes(value as RiskLevel)
    }

    expect(isRiskLevel('safe')).toBe(true)
    expect(isRiskLevel('warning')).toBe(true)
    expect(isRiskLevel('dangerous')).toBe(true)
    expect(isRiskLevel('invalid')).toBe(false)
    expect(isRiskLevel('')).toBe(false)
  })
})

describe('RecommendedAction Enum', () => {
  it('should have all expected enum values', () => {
    expect(RecommendedAction.None).toBe('none')
    expect(RecommendedAction.CanClose).toBe('can_close')
    expect(RecommendedAction.ShouldClose).toBe('should_close')
    expect(RecommendedAction.KeepRunning).toBe('keep_running')
  })

  it('should have exactly 4 values', () => {
    const values = Object.values(RecommendedAction)
    expect(values).toHaveLength(4)
  })

  it('should only contain string values', () => {
    const values = Object.values(RecommendedAction)
    values.forEach((value) => {
      expect(typeof value).toBe('string')
    })
  })

  it('should be usable as type guard', () => {
    function isRecommendedAction(value: string): value is RecommendedAction {
      return Object.values(RecommendedAction).includes(value as RecommendedAction)
    }

    expect(isRecommendedAction('can_close')).toBe(true)
    expect(isRecommendedAction('keep_running')).toBe(true)
    expect(isRecommendedAction('invalid')).toBe(false)
    expect(isRecommendedAction('')).toBe(false)
  })
})

describe('STARTUP_TYPE_LABELS', () => {
  it('should have a label for every StartupType enum value', () => {
    const enumValues = Object.values(StartupType)
    const labelKeys = Object.keys(STARTUP_TYPE_LABELS)

    expect(labelKeys).toHaveLength(enumValues.length)
    enumValues.forEach((value) => {
      expect(STARTUP_TYPE_LABELS).toHaveProperty(value)
    })
  })

  it('should map all values to non-empty strings', () => {
    Object.values(STARTUP_TYPE_LABELS).forEach((label) => {
      expect(typeof label).toBe('string')
      expect(label.length).toBeGreaterThan(0)
    })
  })

  it('should have correct Chinese labels', () => {
    expect(STARTUP_TYPE_LABELS[StartupType.Unknown]).toBe('未知')
    expect(STARTUP_TYPE_LABELS[StartupType.RegistryRun]).toBe('注册表启动')
    expect(STARTUP_TYPE_LABELS[StartupType.RegistryRunOnce]).toBe('注册表启动(一次性)')
    expect(STARTUP_TYPE_LABELS[StartupType.TaskScheduler]).toBe('任务计划')
    expect(STARTUP_TYPE_LABELS[StartupType.WindowsService]).toBe('系统服务')
    expect(STARTUP_TYPE_LABELS[StartupType.StartupFolder]).toBe('启动文件夹')
    expect(STARTUP_TYPE_LABELS[StartupType.Normal]).toBe('用户启动')
  })

  it('should be accessible with bracket notation using enum values', () => {
    expect(STARTUP_TYPE_LABELS['unknown']).toBe('未知')
    expect(STARTUP_TYPE_LABELS['startup_folder']).toBe('启动文件夹')
  })
})

describe('RISK_LEVEL_LABELS', () => {
  it('should have a label for every RiskLevel enum value', () => {
    const enumValues = Object.values(RiskLevel)
    const labelKeys = Object.keys(RISK_LEVEL_LABELS)

    expect(labelKeys).toHaveLength(enumValues.length)
    enumValues.forEach((value) => {
      expect(RISK_LEVEL_LABELS).toHaveProperty(value)
    })
  })

  it('should map all values to non-empty strings', () => {
    Object.values(RISK_LEVEL_LABELS).forEach((label) => {
      expect(typeof label).toBe('string')
      expect(label.length).toBeGreaterThan(0)
    })
  })

  it('should have correct Chinese labels', () => {
    expect(RISK_LEVEL_LABELS[RiskLevel.Safe]).toBe('安全')
    expect(RISK_LEVEL_LABELS[RiskLevel.Low]).toBe('低风险')
    expect(RISK_LEVEL_LABELS[RiskLevel.Caution]).toBe('谨慎')
    expect(RISK_LEVEL_LABELS[RiskLevel.Dangerous]).toBe('危险')
    expect(RISK_LEVEL_LABELS[RiskLevel.Warning]).toBe('警告')
    expect(RISK_LEVEL_LABELS[RiskLevel.Unknown]).toBe('未知')
  })

  it('should be accessible with bracket notation using enum values', () => {
    expect(RISK_LEVEL_LABELS['safe']).toBe('安全')
    expect(RISK_LEVEL_LABELS['dangerous']).toBe('危险')
  })
})

describe('RISK_LEVEL_COLORS', () => {
  it('should have a color for every RiskLevel enum value', () => {
    const enumValues = Object.values(RiskLevel)
    const colorKeys = Object.keys(RISK_LEVEL_COLORS)

    expect(colorKeys).toHaveLength(enumValues.length)
    enumValues.forEach((value) => {
      expect(RISK_LEVEL_COLORS).toHaveProperty(value)
    })
  })

  it('should map all values to non-empty strings', () => {
    Object.values(RISK_LEVEL_COLORS).forEach((color) => {
      expect(typeof color).toBe('string')
      expect(color.length).toBeGreaterThan(0)
    })
  })

  it('should use valid CSS custom properties', () => {
    Object.values(RISK_LEVEL_COLORS).forEach((color) => {
      expect(color).toMatch(/^var\(--color-[a-z-]+\)$/)
    })
  })

  it('should have correct color mappings', () => {
    expect(RISK_LEVEL_COLORS[RiskLevel.Safe]).toBe('var(--color-success)')
    expect(RISK_LEVEL_COLORS[RiskLevel.Low]).toBe('var(--color-risk-low)')
    expect(RISK_LEVEL_COLORS[RiskLevel.Caution]).toBe('var(--color-warning)')
    expect(RISK_LEVEL_COLORS[RiskLevel.Dangerous]).toBe('var(--color-danger)')
    expect(RISK_LEVEL_COLORS[RiskLevel.Warning]).toBe('var(--color-warning)')
    expect(RISK_LEVEL_COLORS[RiskLevel.Unknown]).toBe('var(--color-text-secondary)')
  })

  it('should be accessible with bracket notation using enum values', () => {
    expect(RISK_LEVEL_COLORS['safe']).toBe('var(--color-success)')
    expect(RISK_LEVEL_COLORS['dangerous']).toBe('var(--color-danger)')
  })
})

describe('ProcessInfo Interface', () => {
  it('should be instantiable with all required properties', () => {
    const process: ProcessInfo = {
      pid: 1234,
      name: 'test-process',
      executablePath: 'C:\\Windows\\test.exe',
      cpuUsage: 10.5,
      memoryUsage: 1024000,
      runningTime: 3600,
      startupType: StartupType.Normal,
      isKnownProcess: true,
      riskLevel: RiskLevel.Safe,
      canClose: true,
      recommendedAction: RecommendedAction.None
    }

    expect(process).toBeDefined()
    expect(process.pid).toBe(1234)
    expect(process.name).toBe('test-process')
    expect(process.executablePath).toBe('C:\\Windows\\test.exe')
    expect(process.cpuUsage).toBe(10.5)
    expect(process.memoryUsage).toBe(1024000)
    expect(process.runningTime).toBe(3600)
    expect(process.startupType).toBe(StartupType.Normal)
    expect(process.isKnownProcess).toBe(true)
    expect(process.riskLevel).toBe(RiskLevel.Safe)
    expect(process.canClose).toBe(true)
    expect(process.recommendedAction).toBe(RecommendedAction.None)
  })

  it('should accept optional properties', () => {
    const process: ProcessInfo = {
      pid: 1234,
      name: 'test-process',
      executablePath: 'C:\\Windows\\test.exe',
      publisher: 'Test Corp',
      cpuUsage: 10.5,
      memoryUsage: 1024000,
      runningTime: 3600,
      startupType: StartupType.RegistryRun,
      startupLocation: 'HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run',
      localDescription: 'Test process description',
      isKnownProcess: false,
      riskLevel: RiskLevel.Warning,
      performanceImpact: 'high',
      recommendation: 'Should close to free up resources',
      canClose: true,
      recommendedAction: RecommendedAction.ShouldClose
    }

    expect(process.publisher).toBe('Test Corp')
    expect(process.startupLocation).toBe('HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run')
    expect(process.localDescription).toBe('Test process description')
    expect(process.performanceImpact).toBe('high')
    expect(process.recommendation).toBe('Should close to free up resources')
  })

  it('should work with minimal required properties', () => {
    const process: ProcessInfo = {
      pid: 0,
      name: '',
      executablePath: '',
      cpuUsage: 0,
      memoryUsage: 0,
      runningTime: 0,
      startupType: StartupType.Unknown,
      isKnownProcess: false,
      riskLevel: RiskLevel.Unknown,
      canClose: false,
      recommendedAction: RecommendedAction.None
    }

    expect(process.pid).toBe(0)
    expect(process.name).toBe('')
    expect(process.cpuUsage).toBe(0)
  })

  it('should handle edge case values', () => {
    const process: ProcessInfo = {
      pid: Number.MAX_SAFE_INTEGER,
      name: 'process-with-very-long-name'.repeat(100),
      executablePath: '',
      cpuUsage: Number.MAX_VALUE,
      memoryUsage: Number.MAX_VALUE,
      runningTime: Number.MAX_SAFE_INTEGER,
      startupType: StartupType.WindowsService,
      startupLocation: undefined,
      localDescription: undefined,
      isKnownProcess: true,
      riskLevel: RiskLevel.Caution,
      performanceImpact: 'low',
      recommendation: undefined,
      canClose: false,
      recommendedAction: RecommendedAction.KeepRunning
    }

    expect(process.pid).toBe(Number.MAX_SAFE_INTEGER)
    expect(process.name).toHaveLength(2700)
    expect(process.cpuUsage).toBe(Number.MAX_VALUE)
  })
})

describe('ProcessFilter Interface', () => {
  it('should be instantiable with all optional properties', () => {
    const filter: ProcessFilter = {}
    expect(filter).toBeDefined()
    expect(Object.keys(filter)).toHaveLength(0)
  })

  it('should accept search filter', () => {
    const filter: ProcessFilter = {
      search: 'chrome'
    }
    expect(filter.search).toBe('chrome')
  })

  it('should accept startupTypes filter with multiple values', () => {
    const filter: ProcessFilter = {
      startupTypes: [StartupType.RegistryRun, StartupType.StartupFolder]
    }
    expect(filter.startupTypes).toHaveLength(2)
    expect(filter.startupTypes).toContain(StartupType.RegistryRun)
    expect(filter.startupTypes).toContain(StartupType.StartupFolder)
  })

  it('should accept riskLevels filter with multiple values', () => {
    const filter: ProcessFilter = {
      riskLevels: [RiskLevel.Warning, RiskLevel.Caution]
    }
    expect(filter.riskLevels).toHaveLength(2)
    expect(filter.riskLevels).toContain(RiskLevel.Warning)
    expect(filter.riskLevels).toContain(RiskLevel.Caution)
  })

  it('should accept canCloseOnly filter', () => {
    const filter: ProcessFilter = {
      canCloseOnly: true
    }
    expect(filter.canCloseOnly).toBe(true)
  })

  it('should accept all filters together', () => {
    const filter: ProcessFilter = {
      search: 'test',
      startupTypes: [StartupType.Normal],
      riskLevels: [RiskLevel.Safe],
      canCloseOnly: true
    }

    expect(filter.search).toBe('test')
    expect(filter.startupTypes).toEqual([StartupType.Normal])
    expect(filter.riskLevels).toEqual([RiskLevel.Safe])
    expect(filter.canCloseOnly).toBe(true)
  })

  it('should handle empty arrays', () => {
    const filter: ProcessFilter = {
      startupTypes: [],
      riskLevels: []
    }
    expect(filter.startupTypes).toEqual([])
    expect(filter.riskLevels).toEqual([])
  })

  it('should handle empty string search', () => {
    const filter: ProcessFilter = {
      search: ''
    }
    expect(filter.search).toBe('')
  })

  it('should handle all StartupType and RiskLevel values', () => {
    const filter: ProcessFilter = {
      startupTypes: Object.values(StartupType),
      riskLevels: Object.values(RiskLevel)
    }

    expect(filter.startupTypes).toHaveLength(7)
    expect(filter.riskLevels).toHaveLength(6)
  })
})

describe('Type Interoperability', () => {
  it('should allow using enums in both ProcessInfo and ProcessFilter', () => {
    const startupTypes = [StartupType.RegistryRun, StartupType.TaskScheduler]
    const riskLevels = [RiskLevel.Safe, RiskLevel.Caution]

    const filter: ProcessFilter = {
      startupTypes,
      riskLevels
    }

    const process: ProcessInfo = {
      pid: 1234,
      name: 'test',
      executablePath: 'C:\\test.exe',
      cpuUsage: 0,
      memoryUsage: 0,
      runningTime: 0,
      startupType: StartupType.RegistryRun,
      isKnownProcess: true,
      riskLevel: RiskLevel.Safe,
      canClose: true,
      recommendedAction: RecommendedAction.CanClose
    }

    expect(filter.startupTypes).toContain(process.startupType)
    expect(filter.riskLevels).toContain(process.riskLevel)
  })

  it('should support type narrowing', () => {
    function getRiskColor(level: RiskLevel): string {
      return RISK_LEVEL_COLORS[level]
    }

    function getStartupTypeLabel(type: StartupType): string {
      return STARTUP_TYPE_LABELS[type]
    }

    expect(getRiskColor(RiskLevel.Dangerous)).toBe('var(--color-danger)')
    expect(getRiskColor(RiskLevel.Warning)).toBe('var(--color-warning)')
    expect(getStartupTypeLabel(StartupType.WindowsService)).toBe('系统服务')
  })
})
