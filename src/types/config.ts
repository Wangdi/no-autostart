export interface PermanentAction {
  type: 'disable_startup' | 'delete_task' | 'uninstall'
  description: string
  executedAt: string
  originalLocation?: string
}

export interface AutoCloseItem {
  id: string
  processName: string
  executablePath: string
  addedAt: string
  permanentAction?: PermanentAction
}

export interface AppSettings {
  autoRunOnLogin: boolean
  autoCloseOnStart: boolean
  checkInterval: number
  showNotification: boolean
}

export interface AutoCloseConfig {
  version: string
  lastUpdated: string
  autoCloseList: AutoCloseItem[]
  settings: AppSettings
}

export const DEFAULT_SETTINGS: AppSettings = {
  autoRunOnLogin: true,
  autoCloseOnStart: true,
  checkInterval: 0,
  showNotification: true
}

export const DEFAULT_CONFIG: AutoCloseConfig = {
  version: '1.0.0',
  lastUpdated: new Date().toISOString(),
  autoCloseList: [],
  settings: DEFAULT_SETTINGS
}
