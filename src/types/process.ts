export enum StartupType {
  Unknown = 'unknown',
  RegistryRun = 'registry_run',
  RegistryRunOnce = 'registry_run_once',
  TaskScheduler = 'task_scheduler',
  WindowsService = 'windows_service',
  StartupFolder = 'startup_folder',
  Normal = 'normal'
}

export enum RiskLevel {
  Safe = 'safe',
  Low = 'low',
  Caution = 'caution',
  Dangerous = 'dangerous',
  Warning = 'warning',
  Unknown = 'unknown'
}

export enum RecommendedAction {
  None = 'none',
  CanClose = 'can_close',
  ShouldClose = 'should_close',
  KeepRunning = 'keep_running'
}

export interface ProcessInfo {
  pid: number
  name: string
  executablePath: string
  publisher?: string
  cpuUsage: number
  memoryUsage: number
  runningTime: number
  startupType: StartupType
  startupLocation?: string
  localDescription?: string
  isKnownProcess: boolean
  riskLevel: RiskLevel
  performanceImpact?: string
  recommendation?: string
  canClose: boolean
  recommendedAction?: RecommendedAction
}

export interface ProcessFilter {
  search?: string
  startupTypes?: StartupType[]
  riskLevels?: RiskLevel[]
  canCloseOnly?: boolean
}

export const STARTUP_TYPE_LABELS: Record<StartupType, string> = {
  [StartupType.Unknown]: '未知',
  [StartupType.RegistryRun]: '注册表启动',
  [StartupType.RegistryRunOnce]: '注册表启动(一次性)',
  [StartupType.TaskScheduler]: '任务计划',
  [StartupType.WindowsService]: '系统服务',
  [StartupType.StartupFolder]: '启动文件夹',
  [StartupType.Normal]: '用户启动'
}

export const RISK_LEVEL_LABELS: Record<RiskLevel, string> = {
  [RiskLevel.Safe]: '安全',
  [RiskLevel.Low]: '低风险',
  [RiskLevel.Caution]: '谨慎',
  [RiskLevel.Dangerous]: '危险',
  [RiskLevel.Warning]: '警告',
  [RiskLevel.Unknown]: '未知'
}

export const RISK_LEVEL_COLORS: Record<RiskLevel, string> = {
  [RiskLevel.Safe]: 'var(--color-success)',
  [RiskLevel.Low]: 'var(--color-risk-low)',
  [RiskLevel.Caution]: 'var(--color-warning)',
  [RiskLevel.Dangerous]: 'var(--color-danger)',
  [RiskLevel.Warning]: 'var(--color-warning)',
  [RiskLevel.Unknown]: 'var(--color-text-secondary)'
}
