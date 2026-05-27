import { describe, it, expect } from 'vitest'
import {
  OPERATION_TYPE_LABELS,
  OPERATION_STATUS_LABELS
} from '../history'
import { StartupType } from '../process'
import type { ProcessSnapshot, HistoryRecord, OperationHistory, PermanentActionBackup } from '../history'

describe('OPERATION_TYPE_LABELS', () => {
  it('should have labels for all operation types', () => {
    expect(OPERATION_TYPE_LABELS).toHaveProperty('close_process')
    expect(OPERATION_TYPE_LABELS).toHaveProperty('permanent_close')
    expect(Object.keys(OPERATION_TYPE_LABELS)).toHaveLength(2)
  })

  it('should map to correct Chinese labels', () => {
    expect(OPERATION_TYPE_LABELS.close_process).toBe('关闭进程')
    expect(OPERATION_TYPE_LABELS.permanent_close).toBe('永久关闭')
  })

  it('should return string labels', () => {
    Object.values(OPERATION_TYPE_LABELS).forEach((label) => {
      expect(typeof label).toBe('string')
      expect(label.length).toBeGreaterThan(0)
    })
  })

  it('should be accessible with bracket notation', () => {
    expect(OPERATION_TYPE_LABELS['close_process']).toBe('关闭进程')
    expect(OPERATION_TYPE_LABELS['permanent_close']).toBe('永久关闭')
  })

  it('should be typed as Record<string, string>', () => {
    const type: Record<string, string> = OPERATION_TYPE_LABELS
    expect(type).toBeDefined()
  })
})

describe('OPERATION_STATUS_LABELS', () => {
  it('should have labels for all operation status values', () => {
    expect(OPERATION_STATUS_LABELS).toHaveProperty('completed')
    expect(OPERATION_STATUS_LABELS).toHaveProperty('reverted')
    expect(Object.keys(OPERATION_STATUS_LABELS)).toHaveLength(2)
  })

  it('should map to correct Chinese labels', () => {
    expect(OPERATION_STATUS_LABELS.completed).toBe('已完成')
    expect(OPERATION_STATUS_LABELS.reverted).toBe('已撤销')
  })

  it('should return string labels', () => {
    Object.values(OPERATION_STATUS_LABELS).forEach((label) => {
      expect(typeof label).toBe('string')
      expect(label.length).toBeGreaterThan(0)
    })
  })

  it('should be accessible with bracket notation', () => {
    expect(OPERATION_STATUS_LABELS['completed']).toBe('已完成')
    expect(OPERATION_STATUS_LABELS['reverted']).toBe('已撤销')
  })

  it('should be typed as Record<string, string>', () => {
    const type: Record<string, string> = OPERATION_STATUS_LABELS
    expect(type).toBeDefined()
  })
})

describe('ProcessSnapshot Interface', () => {
  it('should be instantiable with all required properties', () => {
    const snapshot: ProcessSnapshot = {
      pid: 1234,
      name: 'chrome.exe',
      executablePath: 'C:\\Program Files\\Google\\Chrome\\chrome.exe',
      startupType: StartupType.Normal
    }

    expect(snapshot.pid).toBe(1234)
    expect(snapshot.name).toBe('chrome.exe')
    expect(snapshot.executablePath).toBe('C:\\Program Files\\Google\\Chrome\\chrome.exe')
    expect(snapshot.startupType).toBe(StartupType.Normal)
  })

  it('should accept optional startupLocation', () => {
    const snapshot: ProcessSnapshot = {
      pid: 1234,
      name: 'app.exe',
      executablePath: 'C:\\app.exe',
      startupType: StartupType.RegistryRun,
      startupLocation: 'HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run'
    }

    expect(snapshot.startupLocation).toBe('HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run')
  })

  it('should work with zero pid', () => {
    const snapshot: ProcessSnapshot = {
      pid: 0,
      name: '',
      executablePath: '',
      startupType: StartupType.Unknown
    }

    expect(snapshot.pid).toBe(0)
  })

  it('should work with all StartupType values', () => {
    const startupTypes = [
      StartupType.Unknown,
      StartupType.RegistryRun,
      StartupType.RegistryRunOnce,
      StartupType.TaskScheduler,
      StartupType.WindowsService,
      StartupType.StartupFolder,
      StartupType.Normal
    ]

    startupTypes.forEach((type) => {
      const snapshot: ProcessSnapshot = {
        pid: 1,
        name: 'test.exe',
        executablePath: 'C:\\test.exe',
        startupType: type
      }
      expect(snapshot.startupType).toBe(type)
    })
  })

  it('should handle paths with special characters', () => {
    const snapshot: ProcessSnapshot = {
      pid: 1234,
      name: 'app with spaces.exe',
      executablePath: 'C:\\Program Files (x86)\\Test App\\app.exe',
      startupType: StartupType.StartupFolder,
      startupLocation: 'C:\\Users\\Test User\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup'
    }

    expect(snapshot.executablePath).toContain(' (x86)')
    expect(snapshot.startupLocation).toContain(' ')
  })

  it('should handle large pid values', () => {
    const snapshot: ProcessSnapshot = {
      pid: Number.MAX_SAFE_INTEGER,
      name: 'test.exe',
      executablePath: 'C:\\test.exe',
      startupType: StartupType.Normal
    }

    expect(snapshot.pid).toBeLessThan(Number.MAX_VALUE)
    expect(snapshot.pid).toBeGreaterThan(0)
  })
})

describe('PermanentActionBackup Interface', () => {
  it('should accept disable_startup type with backup data', () => {
    const backup: PermanentActionBackup = {
      type: 'disable_startup',
      backupData: {
        registryPath: 'HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run',
        originalValue: 'C:\\app.exe',
        timestamp: '2024-01-15T10:30:00.000Z'
      }
    }

    expect(backup.type).toBe('disable_startup')
    expect(backup.backupData.registryPath).toBe('HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run')
  })

  it('should accept delete_task type with backup data', () => {
    const backup: PermanentActionBackup = {
      type: 'delete_task',
      backupData: {
        taskName: 'MyTask',
        taskXml: '<Task>...</Task>',
        folderPath: '\\\\MyFolder'
      }
    }

    expect(backup.type).toBe('delete_task')
    expect(backup.backupData.taskName).toBe('MyTask')
  })

  it('should accept empty backup data', () => {
    const backup: PermanentActionBackup = {
      type: 'disable_startup',
      backupData: {}
    }

    expect(backup.backupData).toEqual({})
  })

  it('should accept various backup data types', () => {
    const backups: PermanentActionBackup[] = [
      {
        type: 'disable_startup',
        backupData: { key: 'value', number: 123, bool: true }
      },
      {
        type: 'delete_task',
        backupData: { nested: { deep: { value: 'test' } } }
      },
      {
        type: 'disable_startup',
        backupData: { array: [1, 2, 3], nullValue: null }
      }
    ]

    expect(backups[0].backupData.number).toBe(123)
    expect(backups[1].backupData.nested).toEqual({ deep: { value: 'test' } })
  })

  it('should accept only valid type values', () => {
    const disableBackup: PermanentActionBackup = {
      type: 'disable_startup',
      backupData: {}
    }
    const deleteBackup: PermanentActionBackup = {
      type: 'delete_task',
      backupData: {}
    }

    expect(disableBackup.type).toBe('disable_startup')
    expect(deleteBackup.type).toBe('delete_task')
  })
})

describe('HistoryRecord Interface', () => {
  it('should be instantiable with required properties for close_process', () => {
    const record: HistoryRecord = {
      id: 'hist-001',
      timestamp: '2024-01-15T10:30:00.000Z',
      operationType: 'close_process',
      processSnapshot: {
        pid: 1234,
        name: 'chrome.exe',
        executablePath: 'C:\\chrome.exe',
        startupType: StartupType.Normal
      },
      status: 'completed'
    }

    expect(record.id).toBe('hist-001')
    expect(record.operationType).toBe('close_process')
    expect(record.status).toBe('completed')
  })

  it('should be instantiable with required properties for permanent_close', () => {
    const record: HistoryRecord = {
      id: 'hist-002',
      timestamp: '2024-01-15T10:30:00.000Z',
      operationType: 'permanent_close',
      processSnapshot: {
        pid: 5678,
        name: 'app.exe',
        executablePath: 'C:\\app.exe',
        startupType: StartupType.RegistryRun
      },
      status: 'completed'
    }

    expect(record.operationType).toBe('permanent_close')
  })

  it('should accept optional permanentAction', () => {
    const record: HistoryRecord = {
      id: 'hist-003',
      timestamp: '2024-01-15T10:30:00.000Z',
      operationType: 'permanent_close',
      processSnapshot: {
        pid: 9999,
        name: 'service.exe',
        executablePath: 'C:\\service.exe',
        startupType: StartupType.WindowsService
      },
      permanentAction: {
        type: 'disable_startup',
        backupData: {
          originalRegistry: 'HKLM\\Software\\...'
        }
      },
      status: 'completed'
    }

    expect(record.permanentAction).toBeDefined()
    expect(record.permanentAction?.type).toBe('disable_startup')
  })

  it('should accept reverted status with revertedAt timestamp', () => {
    const record: HistoryRecord = {
      id: 'hist-004',
      timestamp: '2024-01-15T10:00:00.000Z',
      operationType: 'permanent_close',
      processSnapshot: {
        pid: 8888,
        name: 'task.exe',
        executablePath: 'C:\\task.exe',
        startupType: StartupType.TaskScheduler
      },
      permanentAction: {
        type: 'delete_task',
        backupData: { taskName: 'MyTask' }
      },
      status: 'reverted',
      revertedAt: '2024-01-15T11:00:00.000Z'
    }

    expect(record.status).toBe('reverted')
    expect(record.revertedAt).toBe('2024-01-15T11:00:00.000Z')
  })

  it('should handle both status values', () => {
    const completed: HistoryRecord = {
      id: '1',
      timestamp: new Date().toISOString(),
      operationType: 'close_process',
      processSnapshot: { pid: 1, name: 'a.exe', executablePath: 'C:\\a.exe', startupType: StartupType.Normal },
      status: 'completed'
    }

    const reverted: HistoryRecord = {
      id: '2',
      timestamp: new Date().toISOString(),
      operationType: 'close_process',
      processSnapshot: { pid: 2, name: 'b.exe', executablePath: 'C:\\b.exe', startupType: StartupType.Normal },
      status: 'reverted',
      revertedAt: new Date().toISOString()
    }

    expect(completed.status).toBe('completed')
    expect(reverted.status).toBe('reverted')
  })

  it('should validate timestamp formats', () => {
    const timestamps = [
      '2024-01-15T10:30:00.000Z',
      '2024-01-15T10:30:00Z',
      '2024-01-15T10:30:00+00:00',
      new Date().toISOString()
    ]

    timestamps.forEach((timestamp) => {
      const record: HistoryRecord = {
        id: 'test',
        timestamp,
        operationType: 'close_process',
        processSnapshot: { pid: 1, name: 'a.exe', executablePath: 'C:\\a.exe', startupType: StartupType.Normal },
        status: 'completed'
      }
      expect(record.timestamp).toBe(timestamp)
    })
  })
})

describe('OperationHistory Interface', () => {
  it('should be instantiable with empty records array', () => {
    const history: OperationHistory = {
      records: []
    }

    expect(history.records).toEqual([])
  })

  it('should accept array of HistoryRecord', () => {
    const records: HistoryRecord[] = [
      {
        id: '1',
        timestamp: '2024-01-15T10:00:00.000Z',
        operationType: 'close_process',
        processSnapshot: { pid: 1, name: 'a.exe', executablePath: 'C:\\a.exe', startupType: StartupType.Normal },
        status: 'completed'
      },
      {
        id: '2',
        timestamp: '2024-01-15T10:01:00.000Z',
        operationType: 'permanent_close',
        processSnapshot: { pid: 2, name: 'b.exe', executablePath: 'C:\\b.exe', startupType: StartupType.RegistryRun },
        permanentAction: { type: 'disable_startup', backupData: {} },
        status: 'reverted',
        revertedAt: '2024-01-15T10:05:00.000Z'
      }
    ]

    const history: OperationHistory = { records }
    expect(history.records).toHaveLength(2)
    expect(history.records[0].status).toBe('completed')
    expect(history.records[1].status).toBe('reverted')
  })

  it('should handle large number of records', () => {
    const records: HistoryRecord[] = Array.from({ length: 1000 }, (_, i) => ({
      id: `record-${i}`,
      timestamp: new Date().toISOString(),
      operationType: i % 2 === 0 ? 'close_process' : 'permanent_close',
      processSnapshot: {
        pid: i,
        name: `process${i}.exe`,
        executablePath: `C:\\process${i}.exe`,
        startupType: StartupType.Normal
      },
      status: 'completed'
    }))

    const history: OperationHistory = { records }
    expect(history.records).toHaveLength(1000)
  })
})

describe('Type Interoperability', () => {
  it('should work with label constants', () => {
    const record: HistoryRecord = {
      id: '1',
      timestamp: new Date().toISOString(),
      operationType: 'close_process',
      processSnapshot: { pid: 1, name: 'a.exe', executablePath: 'C:\\a.exe', startupType: StartupType.Normal },
      status: 'completed'
    }

    const operationLabel = OPERATION_TYPE_LABELS[record.operationType]
    const statusLabel = OPERATION_STATUS_LABELS[record.status]

    expect(operationLabel).toBe('关闭进程')
    expect(statusLabel).toBe('已完成')
  })

  it('should maintain type safety across related interfaces', () => {
    const snapshot: ProcessSnapshot = {
      pid: 1234,
      name: 'test.exe',
      executablePath: 'C:\\test.exe',
      startupType: StartupType.RegistryRun,
      startupLocation: 'HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run'
    }

    const backup: PermanentActionBackup = {
      type: 'disable_startup',
      backupData: { path: snapshot.startupLocation }
    }

    const record: HistoryRecord = {
      id: '1',
      timestamp: new Date().toISOString(),
      operationType: 'permanent_close',
      processSnapshot: snapshot,
      permanentAction: backup,
      status: 'completed'
    }

    const history: OperationHistory = {
      records: [record]
    }

    expect(history.records[0].permanentAction?.type).toBe('disable_startup')
    expect(history.records[0].processSnapshot.startupType).toBe(StartupType.RegistryRun)
  })
})

describe('Edge Cases', () => {
  it('should handle empty strings', () => {
    const record: HistoryRecord = {
      id: '',
      timestamp: '',
      operationType: 'close_process',
      processSnapshot: {
        pid: 0,
        name: '',
        executablePath: '',
        startupType: StartupType.Unknown
      },
      status: 'completed'
    }

    expect(record.id).toBe('')
    expect(record.timestamp).toBe('')
  })

  it('should handle null and undefined in backupData', () => {
    const backup: PermanentActionBackup = {
      type: 'disable_startup',
      backupData: {
        nullValue: null,
        undefinedValue: undefined,
        stringValue: 'test',
        numberValue: 0
      }
    }

    expect(backup.backupData.nullValue).toBeNull()
    expect(backup.backupData.undefinedValue).toBeUndefined()
    expect(backup.backupData.stringValue).toBe('test')
  })

  it('should handle deeply nested backupData', () => {
    const backup: PermanentActionBackup = {
      type: 'disable_startup',
      backupData: {
        level1: {
          level2: {
            level3: {
              value: 'deep'
            }
          }
        }
      }
    }

    expect(backup.backupData.level1.level2.level3.value).toBe('deep')
  })

  it('should handle missing optional fields', () => {
    const record: HistoryRecord = {
      id: '1',
      timestamp: new Date().toISOString(),
      operationType: 'close_process',
      processSnapshot: { pid: 1, name: 'a.exe', executablePath: 'C:\\a.exe', startupType: StartupType.Normal },
      status: 'completed'
    }

    expect(record.permanentAction).toBeUndefined()
    expect(record.revertedAt).toBeUndefined()
  })
})
