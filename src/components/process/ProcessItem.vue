<script setup lang="ts">
import type { ProcessInfo } from '@/types'
import { STARTUP_TYPE_LABELS, RISK_LEVEL_LABELS, RISK_LEVEL_COLORS } from '@/types'

defineProps<{
  process: ProcessInfo
  expanded: boolean
}>()

const emit = defineEmits<{
  toggle: []
  close: []
  addToAutoClose: []
  permanentClose: []
}>()

function formatMemory(bytes: number): string {
  const mb = bytes / (1024 * 1024)
  return mb >= 1 ? `${mb.toFixed(1)} MB` : `${(bytes / 1024).toFixed(0)} KB`
}

function formatDuration(seconds: number): string {
  const hours = Math.floor(seconds / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)
  if (hours > 0) return `${hours}小时${minutes}分`
  if (minutes > 0) return `${minutes}分`
  return `${seconds % 60}秒`
}
</script>

<template>
  <div class="process-item" :class="{ expanded }">
    <div class="process-row" @click="emit('toggle')">
      <span class="expand-icon">{{ expanded ? '▼' : '▶' }}</span>
      <span class="process-name">{{ process.name }}</span>
      <span class="process-pid">{{ process.pid }}</span>
      <span class="process-cpu">{{ process.cpuUsage.toFixed(1) }}%</span>
      <span class="process-memory">{{ formatMemory(process.memoryUsage) }}</span>
      <span class="process-startup">{{ STARTUP_TYPE_LABELS[process.startupType] }}</span>
      <span class="process-action">
        <button
          v-if="process.canClose"
          class="btn-close"
          @click.stop="emit('close')"
        >
          ×
        </button>
        <span v-else class="action-disabled">-</span>
      </span>
    </div>

    <div v-if="expanded" class="process-detail">
      <div class="detail-section">
        <h4>基本信息</h4>
        <div class="detail-grid">
          <div class="detail-item">
            <span class="label">进程名称:</span>
            <span class="value">{{ process.name }}</span>
          </div>
          <div class="detail-item">
            <span class="label">进程ID:</span>
            <span class="value">{{ process.pid }}</span>
          </div>
          <div class="detail-item full-width">
            <span class="label">可执行路径:</span>
            <span class="value">{{ process.executablePath }}</span>
          </div>
          <div class="detail-item">
            <span class="label">发布者:</span>
            <span class="value">{{ process.publisher || '未知' }}</span>
          </div>
          <div class="detail-item">
            <span class="label">运行时长:</span>
            <span class="value">{{ formatDuration(process.runningTime) }}</span>
          </div>
        </div>
      </div>

      <div class="detail-section">
        <h4>风险等级</h4>
        <div class="risk-badge" :style="{ backgroundColor: RISK_LEVEL_COLORS[process.riskLevel] }">
          {{ RISK_LEVEL_LABELS[process.riskLevel] }}
        </div>
      </div>

      <div class="detail-section">
        <h4>操作</h4>
        <div class="action-buttons">
          <button class="btn-action" @click="emit('close')">关闭进程</button>
          <button class="btn-action" @click="emit('addToAutoClose')">加入自动关闭列表</button>
          <button class="btn-action warning" @click="emit('permanentClose')">永久关闭</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.process-item {
  border-bottom: 1px solid var(--color-border);
}

.process-item:hover {
  background: var(--color-bg-secondary);
}

.process-row {
  display: grid;
  grid-template-columns: 24px 1fr 80px 80px 100px 120px 60px;
  align-items: center;
  padding: var(--spacing-sm) var(--spacing-md);
  cursor: pointer;
  gap: var(--spacing-sm);
}

.expand-icon {
  font-size: 12px;
  color: var(--color-text-secondary);
}

.process-name {
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.process-pid,
.process-cpu,
.process-memory {
  text-align: right;
  color: var(--color-text-secondary);
  font-size: 13px;
}

.process-startup {
  font-size: 12px;
  color: var(--color-text-secondary);
}

.btn-close {
  width: 24px;
  height: 24px;
  background: var(--color-danger);
  color: white;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  font-size: 16px;
  line-height: 1;
  transition: background var(--transition-fast);
}

.btn-close:hover {
  background: #dc2626;
}

.action-disabled {
  color: var(--color-text-secondary);
}

.process-detail {
  padding: var(--spacing-md);
  background: var(--color-bg-secondary);
}

.detail-section {
  margin-bottom: var(--spacing-md);
}

.detail-section:last-child {
  margin-bottom: 0;
}

.detail-section h4 {
  font-size: 13px;
  font-weight: 600;
  margin-bottom: var(--spacing-sm);
  color: var(--color-text-secondary);
}

.detail-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--spacing-sm);
}

.detail-item {
  display: flex;
  gap: var(--spacing-sm);
}

.detail-item.full-width {
  grid-column: span 2;
}

.detail-item .label {
  color: var(--color-text-secondary);
  flex-shrink: 0;
}

.detail-item .value {
  color: var(--color-text-primary);
  word-break: break-all;
}

.risk-badge {
  display: inline-block;
  padding: var(--spacing-xs) var(--spacing-md);
  border-radius: var(--radius-sm);
  font-size: 12px;
  font-weight: 500;
  color: white;
}

.action-buttons {
  display: flex;
  gap: var(--spacing-sm);
}

.btn-action {
  padding: var(--spacing-xs) var(--spacing-md);
  background: var(--color-bg-tertiary);
  color: var(--color-text-primary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  cursor: pointer;
  font-size: 13px;
  transition: all var(--transition-fast);
}

.btn-action:hover {
  background: var(--color-accent);
  border-color: var(--color-accent);
}

.btn-action.warning {
  background: var(--color-warning);
  color: #000;
  border-color: var(--color-warning);
}

.btn-action.warning:hover {
  background: #f59e0b;
}
</style>
