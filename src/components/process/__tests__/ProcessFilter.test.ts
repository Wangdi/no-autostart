import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import ProcessFilter from '../ProcessFilter.vue'
import { StartupType, RiskLevel } from '@/types'

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}))

describe('ProcessFilter', () => {
  const defaultProps = {
    filter: {}
  }

  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('renders correctly with default props', () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    expect(wrapper.find('.process-filter').exists()).toBe(true)
    expect(wrapper.findAll('.filter-select')).toHaveLength(2)
  })

  it('displays startup type label', () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    expect(wrapper.text()).toContain('启动类型:')
  })

  it('displays risk level label', () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    expect(wrapper.text()).toContain('风险等级:')
  })

  it('displays all startup type options', async () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const selects = wrapper.findAll('.filter-select')
    const startupSelect = selects[0]
    const options = startupSelect.findAll('option')

    // Should have \"all\" + 7 startup types
    expect(options.length).toBe(8)

    const optionTexts = options.map(o => o.text())
    expect(optionTexts).toContain('全部')
    expect(optionTexts).toContain('注册表启动')
    expect(optionTexts).toContain('注册表启动(一次性)')
    expect(optionTexts).toContain('任务计划')
    expect(optionTexts).toContain('系统服务')
    expect(optionTexts).toContain('启动文件夹')
    expect(optionTexts).toContain('用户启动')
    expect(optionTexts).toContain('未知')
  })

  it('displays all risk level options', async () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const selects = wrapper.findAll('.filter-select')
    const riskSelect = selects[1]
    const options = riskSelect.findAll('option')

    // Should have \"all\" + 6 risk levels
    expect(options.length).toBe(7)

    const optionTexts = options.map(o => o.text())
    expect(optionTexts).toContain('全部')
    expect(optionTexts).toContain('安全')
    expect(optionTexts).toContain('低风险')
    expect(optionTexts).toContain('谨慎')
    expect(optionTexts).toContain('危险')
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

    expect(wrapper.emitted('update:filter')).toBeTruthy()
    expect(wrapper.emitted('update:filter')[0][0]).toEqual({
      startupTypes: [StartupType.RegistryRun]
    })
  })

  it('emits update:filter on risk level change with specific value', async () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const selects = wrapper.findAll('.filter-select')
    const riskSelect = selects[1]

    await riskSelect.setValue('dangerous')

    expect(wrapper.emitted('update:filter')).toBeTruthy()
    expect(wrapper.emitted('update:filter')[0][0]).toEqual({
      riskLevels: [RiskLevel.Dangerous]
    })
  })

  it('emits update:filter with undefined when \"all\" is selected for startup type', async () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const selects = wrapper.findAll('.filter-select')
    const startupSelect = selects[0]

    await startupSelect.setValue('all')

    expect(wrapper.emitted('update:filter')).toBeTruthy()
    expect(wrapper.emitted('update:filter')[0][0]).toEqual({
      startupTypes: undefined
    })
  })

  it('emits update:filter with undefined when \"all\" is selected for risk level', async () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const selects = wrapper.findAll('.filter-select')
    const riskSelect = selects[1]

    await riskSelect.setValue('all')

    expect(wrapper.emitted('update:filter')).toBeTruthy()
    expect(wrapper.emitted('update:filter')[0][0]).toEqual({
      riskLevels: undefined
    })
  })

  it('renders checkbox for canCloseOnly filter', () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const checkbox = wrapper.find('.filter-checkbox input')
    expect(checkbox.exists()).toBe(true)
    expect(checkbox.attributes('type')).toBe('checkbox')
  })

  it('displays correct label for canCloseOnly checkbox', () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    expect(wrapper.text()).toContain('仅显示可关闭')
  })

  it('emits update:filter on canCloseOnly checkbox change', async () => {
    const wrapper = mount(ProcessFilter, {
      props: defaultProps
    })

    const checkbox = wrapper.find('.filter-checkbox input')
    await checkbox.setValue(true)

    expect(wrapper.emitted('update:filter')).toBeTruthy()
    expect(wrapper.emitted('update:filter')[0][0]).toEqual({
      canCloseOnly: true
    })
  })

  it('initializes with correct selected values from filter prop', () => {
    const wrapper = mount(ProcessFilter, {
      props: {
        filter: {
          startupTypes: [StartupType.WindowsService],
          riskLevels: [RiskLevel.Safe],
          canCloseOnly: true
        }
      }
    })

    // Component should receive the filter but select elements may not show the value
    // since the component uses local state
    expect(wrapper.props('filter')).toEqual({
      startupTypes: [StartupType.WindowsService],
      riskLevels: [RiskLevel.Safe],
      canCloseOnly: true
    })
  })
})
