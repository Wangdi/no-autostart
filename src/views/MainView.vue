<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useProcessStore, useConfigStore } from '@/stores'
import SearchBar from '@/components/process/SearchBar.vue'
import ProcessFilterComponent from '@/components/process/ProcessFilter.vue'
import ProcessList from '@/components/process/ProcessList.vue'
import type { ProcessInfo, ProcessFilter } from '@/types'

const processStore = useProcessStore()
const configStore = useConfigStore()

const searchQuery = ref('')

onMounted(async () => {
  await processStore.fetchProcesses()
  await configStore.loadConfig()
})

function handleRefresh() {
  processStore.fetchProcesses()
}

function handleSearch(value: string) {
  processStore.setFilter({ search: value })
}

function handleFilterChange(filter: Partial<ProcessFilter>) {
  processStore.setFilter(filter)
}

async function handleClose(pid: number) {
  await processStore.closeProcess(pid)
}

async function handleAddToAutoClose(process: ProcessInfo) {
  await configStore.addToAutoCloseList({
    processName: process.name,
    executablePath: process.executablePath,
  })
}

function handlePermanentClose(process: ProcessInfo) {
  // Will implement modal in next task
  console.log('Permanent close:', process.name)
}
</script>

<template>
  <div class="main-view">
    <header class="header">
      <h1 class="title">NoAutoStart</h1>
      <button class="btn-settings" @click="() => {}">设置</button>
    </header>

    <div class="toolbar">
      <SearchBar
        v-model="searchQuery"
        @update:model-value="handleSearch"
        @refresh="handleRefresh"
      />
    </div>

    <div class="filter-bar">
      <ProcessFilterComponent
        :filter="processStore.filter"
        @update:filter="handleFilterChange"
      />
    </div>

    <main class="content">
      <ProcessList
        :processes="processStore.filteredProcesses"
        :loading="processStore.loading"
        @close="handleClose"
        @add-to-auto-close="handleAddToAutoClose"
        @permanent-close="handlePermanentClose"
      />
    </main>

    <footer class="footer">
      <span>自动关闭列表: {{ configStore.config.autoCloseList.length }}个</span>
    </footer>
  </div>
</template>

<style scoped>
.main-view {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md);
  background: var(--color-bg-secondary);
  border-bottom: 1px solid var(--color-border);
}

.title {
  font-size: 18px;
  font-weight: 600;
}

.btn-settings {
  padding: var(--spacing-xs) var(--spacing-md);
  background: var(--color-bg-tertiary);
  color: var(--color-text-primary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  cursor: pointer;
  font-size: 13px;
}

.btn-settings:hover {
  background: var(--color-accent);
}

.toolbar {
  padding: var(--spacing-sm) var(--spacing-md);
}

.filter-bar {
  padding: 0 var(--spacing-md) var(--spacing-sm);
}

.content {
  flex: 1;
  padding: 0 var(--spacing-md);
  overflow: hidden;
}

.footer {
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--color-bg-secondary);
  border-top: 1px solid var(--color-border);
  font-size: 12px;
  color: var(--color-text-secondary);
}
</style>
