import { describe, it, expect } from 'vitest'
import {
  DEFAULT_SETTINGS,
  DEFAULT_CONFIG
} from '../config'
import type { AutoCloseItem, AutoCloseConfig, AppSettings, PermanentAction } from '../config'

describe('DEFAULT_SETTINGS', () => {
  it('should have correct default values', () => {
    expect(DEFAULT_SETTINGS.autoRunOnLogin).toBe(true)
    expect(DEFAULT_SETTINGS.autoCloseOnStart).toBe(true)
    expect(DEFAULT_SETTINGS.checkInterval).toBe(0)
    expect(DEFAULT_SETTINGS.showNotification).toBe(true)
  })

  it('should satisfy AppSettings interface', () => {
    const settings: AppSettings = DEFAULT_SETTINGS
    expect(settings).toBeDefined()
    expect(typeof settings.autoRunOnLogin).toBe('boolean')
    expect(typeof settings.autoCloseOnStart).toBe('boolean')
    expect(typeof settings.checkInterval).toBe('number')
    expect(typeof settings.showNotification).toBe('boolean')
  })

  it('should have immutable default values', () => {
    const original = { ...DEFAULT_SETTINGS }
    const modified = { ...DEFAULT_SETTINGS, checkInterval: 1000 }

    expect(DEFAULT_SETTINGS.checkInterval).toBe(0)
    expect(modified.checkInterval).toBe(1000)
    expect(DEFAULT_SETTINGS).toEqual(original)
  })
})

describe('DEFAULT_CONFIG', () => {
  it('should have correct version', () => {
    expect(DEFAULT_CONFIG.version).toBe('1.0.0')
  })

  it('should have valid ISO string for lastUpdated', () => {
    expect(typeof DEFAULT_CONFIG.lastUpdated).toBe('string')
    expect(DEFAULT_CONFIG.lastUpdated).toMatch(/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}/)
  })

  it('should have empty autoCloseList array', () => {
    expect(Array.isArray(DEFAULT_CONFIG.autoCloseList)).toBe(true)
    expect(DEFAULT_CONFIG.autoCloseList).toHaveLength(0)
  })

  it('should reference DEFAULT_SETTINGS for settings', () => {
    expect(DEFAULT_CONFIG.settings).toBe(DEFAULT_SETTINGS)
  })

  it('should satisfy AutoCloseConfig interface', () => {
    const config: AutoCloseConfig = DEFAULT_CONFIG
    expect(config).toBeDefined()
    expect(typeof config.version).toBe('string')
    expect(typeof config.lastUpdated).toBe('string')
    expect(Array.isArray(config.autoCloseList)).toBe(true)
    expect(typeof config.settings).toBe('object')
  })
})

describe('AppSettings Interface', () => {
  it('should accept all boolean combinations for autoRunOnLogin', () => {
    const withTrue: AppSettings = {
      autoRunOnLogin: true,
      autoCloseOnStart: true,
      checkInterval: 0,
      showNotification: true
    }
    const withFalse: AppSettings = {
      autoRunOnLogin: false,
      autoCloseOnStart: true,
      checkInterval: 0,
      showNotification: true
    }

    expect(withTrue.autoRunOnLogin).toBe(true)
    expect(withFalse.autoRunOnLogin).toBe(false)
  })

  it('should accept all boolean combinations for autoCloseOnStart', () => {
    const withTrue: AppSettings = {
      autoRunOnLogin: true,
      autoCloseOnStart: true,
      checkInterval: 0,
      showNotification: true
    }
    const withFalse: AppSettings = {
      autoRunOnLogin: true,
      autoCloseOnStart: false,
      checkInterval: 0,
      showNotification: true
    }

    expect(withTrue.autoCloseOnStart).toBe(true)
    expect(withFalse.autoCloseOnStart).toBe(false)
  })

  it('should accept various checkInterval values', () => {
    const zero: AppSettings = {
      autoRunOnLogin: true,
      autoCloseOnStart: true,
      checkInterval: 0,
      showNotification: true
    }
    const positive: AppSettings = {
      autoRunOnLogin: true,
      autoCloseOnStart: true,
      checkInterval: 5000,
      showNotification: true
    }
    const large: AppSettings = {
      autoRunOnLogin: true,
      autoCloseOnStart: true,
      checkInterval: 86400000,
      showNotification: true
    }

    expect(zero.checkInterval).toBe(0)
    expect(positive.checkInterval).toBe(5000)
    expect(large.checkInterval).toBe(86400000)
  })

  it('should accept all boolean combinations for showNotification', () => {
    const withTrue: AppSettings = {
      autoRunOnLogin: true,
      autoCloseOnStart: true,
      checkInterval: 0,
      showNotification: true
    }
    const withFalse: AppSettings = {
      autoRunOnLogin: true,
      autoCloseOnStart: true,
      checkInterval: 0,
      showNotification: false
    }

    expect(withTrue.showNotification).toBe(true)
    expect(withFalse.showNotification).toBe(false)
  })

  it('should handle zero checkInterval', () => {
    const settings: AppSettings = {
      autoRunOnLogin: true,
      autoCloseOnStart: true,
      checkInterval: 0,
      showNotification: true
    }
    expect(settings.checkInterval).toBe(0)
  })

  it('should handle negative checkInterval (edge case)', () => {
    const settings: AppSettings = {
      autoRunOnLogin: true,
      autoCloseOnStart: true,
      checkInterval: -1,
      showNotification: true
    }
    expect(settings.checkInterval).toBe(-1)
  })
})

describe('PermanentAction Interface', () => {
  it('should accept disable_startup type', () => {
    const action: PermanentAction = {
      type: 'disable_startup',
      description: 'Disable from startup registry',
      executedAt: '2024-01-15T10:30:00.000Z',
      originalLocation: 'HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run'
    }

    expect(action.type).toBe('disable_startup')
    expect(action.description).toBe('Disable from startup registry')
    expect(action.executedAt).toBe('2024-01-15T10:30:00.000Z')
    expect(action.originalLocation).toBe('HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run')
  })

  it('should accept delete_task type', () => {
    const action: PermanentAction = {
      type: 'delete_task',
      description: 'Delete scheduled task',
      executedAt: '2024-01-15T10:30:00.000Z'
    }

    expect(action.type).toBe('delete_task')
    expect(action.originalLocation).toBeUndefined()
  })

  it('should accept uninstall type', () => {
    const action: PermanentAction = {
      type: 'uninstall',
      description: 'Uninstall application',
      executedAt: '2024-01-15T10:30:00.000Z',
      originalLocation: 'C:\\Program Files\\App'
    }

    expect(action.type).toBe('uninstall')
  })

  it('should require executedAt in ISO format', () => {
    const action: PermanentAction = {
      type: 'disable_startup',
      description: 'Test',
      executedAt: new Date().toISOString()
    }

    expect(action.executedAt).toMatch(/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}/)
  })
})

describe('AutoCloseItem Interface', () => {
  it('should be instantiable with all required properties', () => {
    const item: AutoCloseItem = {
      id: 'item-001',
      processName: 'chrome.exe',
      executablePath: 'C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe',
      addedAt: '2024-01-15T10:30:00.000Z'
    }

    expect(item.id).toBe('item-001')
    expect(item.processName).toBe('chrome.exe')
    expect(item.executablePath).toBe('C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe')
    expect(item.addedAt).toBe('2024-01-15T10:30:00.000Z')
    expect(item.permanentAction).toBeUndefined()
  })

  it('should accept optional permanentAction', () => {
    const item: AutoCloseItem = {
      id: 'item-002',
      processName: 'notepad.exe',
      executablePath: 'C:\\Windows\\System32\\notepad.exe',
      addedAt: '2024-01-15T10:30:00.000Z',
      permanentAction: {
        type: 'disable_startup',
        description: 'Remove from startup',
        executedAt: '2024-01-15T10:30:00.000Z'
      }
    }

    expect(item.permanentAction).toBeDefined()
    expect(item.permanentAction?.type).toBe('disable_startup')
  })

  it('should handle empty strings', () => {
    const item: AutoCloseItem = {
      id: '',
      processName: '',
      executablePath: '',
      addedAt: ''
    }

    expect(item.id).toBe('')
    expect(item.processName).toBe('')
  })

  it('should handle special characters in paths', () => {
    const item: AutoCloseItem = {
      id: 'item-003',
      processName: 'app with spaces.exe',
      executablePath: 'C:\\Users\\Test User\\App Data\\Local\\app.exe',
      addedAt: '2024-01-15T10:30:00.000Z'
    }

    expect(item.processName).toBe('app with spaces.exe')
    expect(item.executablePath).toContain(' ')
  })
})

describe('AutoCloseConfig Interface', () => {
  it('should be instantiable with all properties', () => {
    const settings: AppSettings = {
      autoRunOnLogin: true,
      autoCloseOnStart: true,
      checkInterval: 5000,
      showNotification: true
    }

    const config: AutoCloseConfig = {
      version: '1.0.0',
      lastUpdated: new Date().toISOString(),
      autoCloseList: [],
      settings
    }

    expect(config.version).toBe('1.0.0')
    expect(config.autoCloseList).toEqual([])
    expect(config.settings).toBe(settings)
  })

  it('should accept populated autoCloseList', () => {
    const items: AutoCloseItem[] = [
      {
        id: '1',
        processName: 'a.exe',
        executablePath: 'C:\\a.exe',
        addedAt: '2024-01-01T00:00:00Z'
      },
      {
        id: '2',
        processName: 'b.exe',
        executablePath: 'C:\\b.exe',
        addedAt: '2024-01-02T00:00:00Z',
        permanentAction: {
          type: 'delete_task',
          description: 'Deleted',
          executedAt: '2024-01-02T00:00:00Z'
        }
      }
    ]

    const config: AutoCloseConfig = {
      version: '1.1.0',
      lastUpdated: '2024-01-15T10:30:00Z',
      autoCloseList: items,
      settings: DEFAULT_SETTINGS
    }

    expect(config.autoCloseList).toHaveLength(2)
    expect(config.autoCloseList[0].id).toBe('1')
    expect(config.autoCloseList[1].permanentAction?.type).toBe('delete_task')
  })

  it('should accept different version formats', () => {
    const versions = ['1.0.0', '0.0.1', '2.5.3-alpha', '1.0.0-beta.1']

    versions.forEach((version) => {
      const config: AutoCloseConfig = {
        version,
        lastUpdated: new Date().toISOString(),
        autoCloseList: [],
        settings: DEFAULT_SETTINGS
      }
      expect(config.version).toBe(version)
    })
  })

  it('should reference settings correctly', () => {
    const customSettings: AppSettings = {
      autoRunOnLogin: false,
      autoCloseOnStart: false,
      checkInterval: 10000,
      showNotification: false
    }

    const config: AutoCloseConfig = {
      version: '1.0.0',
      lastUpdated: new Date().toISOString(),
      autoCloseList: [],
      settings: customSettings
    }

    expect(config.settings.autoRunOnLogin).toBe(false)
    expect(config.settings.checkInterval).toBe(10000)
  })
})

describe('Type Exports', () => {
  it('should export all types used in the config module', () => {
    const settings: AppSettings = {
      autoRunOnLogin: true,
      autoCloseOnStart: true,
      checkInterval: 0,
      showNotification: true
    }

    const action: PermanentAction = {
      type: 'disable_startup',
      description: 'Test',
      executedAt: new Date().toISOString()
    }

    const item: AutoCloseItem = {
      id: '1',
      processName: 'test.exe',
      executablePath: 'C:\\test.exe',
      addedAt: new Date().toISOString(),
      permanentAction: action
    }

    const config: AutoCloseConfig = {
      version: '1.0.0',
      lastUpdated: new Date().toISOString(),
      autoCloseList: [item],
      settings
    }

    expect(config).toBeDefined()
    expect(config.autoCloseList[0].permanentAction).toEqual(action)
  })
})

describe('Edge Cases', () => {
  it('should handle very long strings', () => {
    const longPath = 'C:\\' + 'very'.repeat(100) + '\\path.exe'
    const item: AutoCloseItem = {
      id: 'x'.repeat(1000),
      processName: 'a'.repeat(500),
      executablePath: longPath,
      addedAt: new Date().toISOString()
    }

    expect(item.id).toHaveLength(1000)
    expect(item.processName).toHaveLength(500)
  })

  it('should handle various timestamp formats', () => {
    const timestamps = [
      '2024-01-15T10:30:00.000Z',
      '2024-01-15T10:30:00Z',
      '2024-01-15T10:30:00+08:00',
      new Date().toISOString()
    ]

    timestamps.forEach((timestamp) => {
      const action: PermanentAction = {
        type: 'uninstall',
        description: 'Test',
        executedAt: timestamp
      }
      expect(action.executedAt).toBe(timestamp)
    })
  })

  it('should handle large autoCloseList', () => {
    const items: AutoCloseItem[] = Array.from({ length: 1000 }, (_, i) => ({
      id: `item-${i}`,
      processName: `process${i}.exe`,
      executablePath: `C:\\process${i}.exe`,
      addedAt: new Date().toISOString()
    }))

    const config: AutoCloseConfig = {
      version: '1.0.0',
      lastUpdated: new Date().toISOString(),
      autoCloseList: items,
      settings: DEFAULT_SETTINGS
    }

    expect(config.autoCloseList).toHaveLength(1000)
  })
})
