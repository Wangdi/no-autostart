import { StartupType } from './process'

export interface ProcessSnapshot {
  pid: number
  name: string
  executablePath: string
  startupType: StartupType
  startupLocation?: string
}

export interface PermanentActionBackup {
  type: 'disable_startup' | 'delete_task'
  backupData: Record<string, unknown>
}

export interface HistoryRecord {
  id: string
  timestamp: string
  operationType: 'close_process' | 'permanent_close'
  processSnapshot: ProcessSnapshot
  permanentAction?: PermanentActionBackup
  status: 'completed' | 'reverted'
  revertedAt?: string
}

export interface OperationHistory {
  records: HistoryRecord[]
}

export const OPERATION_TYPE_LABELS: Record<string, string> = {
  close_process: '关闭进程',
  permanent_close: '永久关闭'
}

export const OPERATION_STATUS_LABELS: Record<string, string> = {
  completed: '已完成',
  reverted: '已撤销'
}
