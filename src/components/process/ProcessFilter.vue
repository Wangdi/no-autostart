<script setup lang="ts">
import { ref, computed } from 'vue'
import type { ProcessFilter } from '@/types'
import { StartupType, RiskLevel, STARTUP_TYPE_LABELS, RISK_LEVEL_LABELS } from '@/types'

defineProps<{
  filter: ProcessFilter
}>()

const emit = defineEmits<{
  'update:filter': [filter: Partial<ProcessFilter>]
}>()

const startupTypeOptions = computed(() =>
  Object.entries(STARTUP_TYPE_LABELS).map(([value, label]) => ({ value, label }))
)

const riskLevelOptions = computed(() =>
  Object.entries(RISK_LEVEL_LABELS).map(([value, label]) => ({ value, label }))
)

const selectedStartupType = ref<string>('all')
const selectedRiskLevel = ref<string>('all')
const canCloseOnly = ref(false)

function onStartupTypeChange(value: string) {
  selectedStartupType.value = value
  emit('update:filter', {
    startupTypes: value === 'all' ? undefined : [value as StartupType]
  })
}

function onRiskLevelChange(value: string) {
  selectedRiskLevel.value = value
  emit('update:filter', {
    riskLevels: value === 'all' ? undefined : [value as RiskLevel]
  })
}

function onCanCloseChange(value: boolean) {
  canCloseOnly.value = value
  emit('update:filter', { canCloseOnly: value })
}
</script>

<template>
  <div class="process-filter">
    <div class="filter-group">
      <label>启动类型:</label>
      <select
        :value="selectedStartupType"
        @change="onStartupTypeChange(($event.target as HTMLSelectElement).value)"
        class="filter-select"
      >
        <option value="all">全部</option>
        <option
          v-for="opt in startupTypeOptions"
          :key="opt.value"
          :value="opt.value"
        >
          {{ opt.label }}
        </option>
      </select>
    </div>

    <div class="filter-group">
      <label>风险等级:</label>
      <select
        :value="selectedRiskLevel"
        @change="onRiskLevelChange(($event.target as HTMLSelectElement).value)"
        class="filter-select"
      >
        <option value="all">全部</option>
        <option
          v-for="opt in riskLevelOptions"
          :key="opt.value"
          :value="opt.value"
        >
          {{ opt.label }}
        </option>
      </select>
    </div>

    <label class="filter-checkbox">
      <input
        type="checkbox"
        :checked="canCloseOnly"
        @change="onCanCloseChange(($event.target as HTMLInputElement).checked)"
      />
      仅显示可关闭
    </label>
  </div>
</template>

<style scoped>
.process-filter {
  display: flex;
  align-items: center;
  gap: var(--spacing-lg);
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--color-bg-secondary);
  border-radius: var(--radius-md);
  font-size: 13px;
}

.filter-group {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.filter-group label {
  color: var(--color-text-secondary);
}

.filter-select {
  padding: var(--spacing-xs) var(--spacing-sm);
  background: var(--color-bg-tertiary);
  color: var(--color-text-primary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.filter-checkbox {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  cursor: pointer;
  user-select: none;
}

.filter-checkbox input {
  cursor: pointer;
}
</style>
