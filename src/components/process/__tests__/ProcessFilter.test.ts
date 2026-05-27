import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import ProcessFilter from '../ProcessFilter.vue'
import type { ProcessFilter as FilterType } from '@/types'

describe('ProcessFilter', () => {
  const defaultProps = {
    filter: {} as FilterType
  }

  it('renders all filter controls', () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    // Check for startup type select
    expect(wrapper.findAll('.filter-group')).toHaveLength(2)
    expect(wrapper.findAll('.filter-select')).toHaveLength(2)

    // Check for checkbox
    expect(wrapper.find('.filter-checkbox').exists()).toBe(true)
    expect(wrapper.find('input[type="checkbox"]').exists()).toBe(true)

    // Check for labels
    expect(wrapper.text()).toContain('启动类型:')
    expect(wrapper.text()).toContain('风险等级:')
    expect(wrapper.text()).toContain('仅显示可关闭')
  })

  it('renders default "all" option for startup type', () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const selects = wrapper.findAll('.filter-select')
    const startupSelect = selects[0]
    const options = startupSelect.findAll('option')

    expect(options[0].attributes('value')).toBe('all')
    expect(options[0].text()).toBe('全部')
  })

  it('renders default "all" option for risk level', () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const selects = wrapper.findAll('.filter-select')
    const riskSelect = selects[1]
    const options = riskSelect.findAll('option')

    expect(options[0].attributes('value')).toBe('all')
    expect(options[0].text()).toBe('全部')
  })

  it('displays all startup type options', () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const selects = wrapper.findAll('.filter-select')
    const startupSelect = selects[0]
    const options = startupSelect.findAll('option')

    // Should have "all" + 7 startup types
    expect(options.length).toBe(8)

    const optionTexts = options.map(o => o.text())
    expect(optionTexts).toContain('全部')
    expect(optionTexts).toContain('未知')
    expect(optionTexts).toContain('注册表启动')
    expect(optionTexts).toContain('注册表启动(一次性)')
    expect(optionTexts).toContain('任务计划')
    expect(optionTexts).toContain('系统服务')
    expect(optionTexts).toContain('启动文件夹')
    expect(optionTexts).toContain('用户启动')
  })

  it('displays all risk level options', () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const selects = wrapper.findAll('.filter-select')
    const riskSelect = selects[1]
    const options = riskSelect.findAll('option')

    // Should have "all" + 4 risk levels
    expect(options.length).toBe(5)

    const optionTexts = options.map(o => o.text())
    expect(optionTexts).toContain('全部')
    expect(optionTexts).toContain('安全')
    expect(optionTexts).toContain('谨慎')
    expect(optionTexts).toContain('警告')
    expect(optionTexts).toContain('未知')
  })

  it('emits update:filter on startup type change with specific value', async () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const selects = wrapper.findAll('.filter-select')
    const startupSelect = selects[0]

    await startupSelect.setValue('registry_run')

    expect(wrapper.emitted('update:filter')).toHaveLength(1)
    expect(wrapper.emitted('update:filter')![0]).toEqual([
      { startupTypes: ['registry_run'] }
    ])
  })

  it('emits update:filter on startup type change with all value', async () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const selects = wrapper.findAll('.filter-select')
    const startupSelect = selects[0]

    // First set to a specific value
    await startupSelect.setValue('registry_run')
    // Then set back to all
    await startupSelect.setValue('all')

    expect(wrapper.emitted('update:filter')).toHaveLength(2)
    expect(wrapper.emitted('update:filter')![1]).toEqual([
      { startupTypes: undefined }
    ])
  })

  it('emits update:filter on risk level change with specific value', async () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const selects = wrapper.findAll('.filter-select')
    const riskSelect = selects[1]

    await riskSelect.setValue('warning')

    expect(wrapper.emitted('update:filter')).toHaveLength(1)
    expect(wrapper.emitted('update:filter')![0]).toEqual([
      { riskLevels: ['warning'] }
    ])
  })

  it('emits update:filter on risk level change with all value', async () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const selects = wrapper.findAll('.filter-select')
    const riskSelect = selects[1]

    // First set to a specific value
    await riskSelect.setValue('safe')
    // Then set back to all
    await riskSelect.setValue('all')

    expect(wrapper.emitted('update:filter')).toHaveLength(2)
    expect(wrapper.emitted('update:filter')![1]).toEqual([
      { riskLevels: undefined }
    ])
  })

  it('emits correct filter on canCloseOnly toggle to true', async () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const checkbox = wrapper.find('input[type="checkbox"]')
    await checkbox.setValue(true)

    expect(wrapper.emitted('update:filter')).toHaveLength(1)
    expect(wrapper.emitted('update:filter')![0]).toEqual([
      { canCloseOnly: true }
    ])
  })

  it('emits correct filter on canCloseOnly toggle to false', async () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const checkbox = wrapper.find('input[type="checkbox"]')
    // First check it
    await checkbox.setValue(true)
    // Then uncheck it
    await checkbox.setValue(false)

    expect(wrapper.emitted('update:filter')).toHaveLength(2)
    expect(wrapper.emitted('update:filter')![1]).toEqual([
      { canCloseOnly: false }
    ])
  })

  it('maintains local state for startup type selection', async () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const selects = wrapper.findAll('.filter-select')
    const startupSelect = selects[0]

    await startupSelect.setValue('task_scheduler')

    expect((startupSelect.element as HTMLSelectElement).value).toBe('task_scheduler')
  })

  it('maintains local state for risk level selection', async () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const selects = wrapper.findAll('.filter-select')
    const riskSelect = selects[1]

    await riskSelect.setValue('caution')

    expect((riskSelect.element as HTMLSelectElement).value).toBe('caution')
  })

  it('maintains local state for checkbox', async () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const checkbox = wrapper.find('input[type="checkbox"]')
    await checkbox.setValue(true)

    expect((checkbox.element as HTMLInputElement).checked).toBe(true)
  })
})
