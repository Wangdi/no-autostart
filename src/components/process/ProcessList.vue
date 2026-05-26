<script setup lang="ts">
import { ref } from 'vue'
import type { ProcessInfo } from '@/types'
import ProcessItem from './ProcessItem.vue'

defineProps<{
  processes: ProcessInfo[]
  loading: boolean
}>()

const emit = defineEmits<{
  close: [pid: number]
  addToAutoClose: [process: ProcessInfo]
  permanentClose: [process: ProcessInfo]
}>()

const expandedPids = ref<Set<number>>(new Set())

function toggleExpand(pid: number) {
  if (expandedPids.value.has(pid)) {
    expandedPids.value.delete(pid)
  } else {
    expandedPids.value.add(pid)
  }
}
</script>

<template>
  <div class="process-list">
    <div class="list-header">
      <span class="col-expand"></span>
      <span class="col-name">名称</span>
      <span class="col-pid">PID</span>
      <span class="col-cpu">CPU</span>
      <span class="col-memory">内存</span>
      <span class="col-startup">启动类型</span>
      <span class="col-action">操作</span>
    </div>

    <div class="list-body">
      <div v-if="loading" class="loading">
        加载中...
      </div>
      <div v-else-if="processes.length === 0" class="empty">
        没有找到进程
      </div>
      <template v-else>
        <ProcessItem
          v-for="process in processes"
          :key="process.pid"
          :process="process"
          :expanded="expandedPids.has(process.pid)"
          @toggle="toggleExpand(process.pid)"
          @close="emit('close', process.pid)"
          @add-to-auto-close="emit('addToAutoClose', process)"
          @permanent-close="emit('permanentClose', process)"
        />
      </template>
    </div>
  </div>
</template>

<style scoped>
.process-list {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: var(--color-bg-primary);
  border-radius: var(--radius-md);
}

.list-header {
  display: grid;
  grid-template-columns: 24px 1fr 80px 80px 100px 120px 60px;
  align-items: center;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--color-bg-tertiary);
  font-size: 12px;
  font-weight: 600;
  color: var(--color-text-secondary);
  gap: var(--spacing-sm);
}

.list-body {
  flex: 1;
  overflow-y: auto;
}

.loading,
.empty {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 200px;
  color: var(--color-text-secondary);
}

.col-expand {
  width: 24px;
}

.col-pid,
.col-cpu,
.col-memory {
  text-align: right;
}
</style>
