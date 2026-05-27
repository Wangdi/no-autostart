import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import ProcessList from '../ProcessList.vue'
import ProcessItem from '../ProcessItem.vue'
import { StartupType, RiskLevel, RecommendedAction } from '@/types'
import type { ProcessInfo } from '@/types'

describe('ProcessList', () => {
  const createMockProcess = (pid: number, name: string): ProcessInfo => ({
    pid,
    name,
    executablePath: `C:\\Program Files\\${name}`,
    publisher: 'Test Publisher',
    cpuUsage: 5.5,
    memoryUsage: 102400000,
    runningTime: 3665,
    startupType: StartupType.Normal,
    isKnownProcess: true,
    riskLevel: RiskLevel.Safe,
    canClose: true,
    recommendedAction: RecommendedAction.CanClose
  })

  const mockProcesses: ProcessInfo[] = [
    createMockProcess(1234, 'process1.exe'),
    createMockProcess(5678, 'process2.exe')
  ]

  it('renders header row with correct columns', () => {
    const wrapper = mount(ProcessList, {
      props: {
        processes: mockProcesses,
        loading: false
      }
    })

    const header = wrapper.find('.list-header')
    expect(header.exists()).toBe(true)

    expect(wrapper.text()).toContain('名称')
    expect(wrapper.text()).toContain('PID')
    expect(wrapper.text()).toContain('CPU')
    expect(wrapper.text()).toContain('内存')
    expect(wrapper.text()).toContain('启动类型')
    expect(wrapper.text()).toContain('操作')
  })

  it('shows loading state when loading prop is true', () => {
    const wrapper = mount(ProcessList, {
      props: {
        processes: [],
        loading: true
      }
    })

    expect(wrapper.find('.loading').exists()).toBe(true)
    expect(wrapper.find('.loading').text()).toBe('加载中...')
    expect(wrapper.find('.empty').exists()).toBe(false)
    expect(wrapper.findAllComponents(ProcessItem)).toHaveLength(0)
  })

  it('shows empty state when processes array is empty and not loading', () => {
    const wrapper = mount(ProcessList, {
      props: {
        processes: [],
        loading: false
      }
    })

    expect(wrapper.find('.empty').exists()).toBe(true)
    expect(wrapper.find('.empty').text()).toBe('没有找到进程')
    expect(wrapper.find('.loading').exists()).toBe(false)
    expect(wrapper.findAllComponents(ProcessItem)).toHaveLength(0)
  })

  it('renders ProcessItem for each process', () => {
    const wrapper = mount(ProcessList, {
      props: {
        processes: mockProcesses,
        loading: false
      }
    })

    const items = wrapper.findAllComponents(ProcessItem)
    expect(items).toHaveLength(2)
  })

  it('passes correct props to ProcessItem', () => {
    const wrapper = mount(ProcessList, {
      props: {
        processes: mockProcesses,
        loading: false
      }
    })

    const items = wrapper.findAllComponents(ProcessItem)

    // Check first process item props
    expect(items[0].props('process')).toEqual(mockProcesses[0])
    expect(items[0].props('expanded')).toBe(false)

    // Check second process item props
    expect(items[1].props('process')).toEqual(mockProcesses[1])
    expect(items[1].props('expanded')).toBe(false)
  })

  it('does not show empty state when loading', () => {
    const wrapper = mount(ProcessList, {
      props: {
        processes: [],
        loading: true
      }
    })

    // Loading takes precedence
    expect(wrapper.find('.loading').exists()).toBe(true)
    expect(wrapper.find('.empty').exists()).toBe(false)
  })

  it('does not show empty state when there are processes', () => {
    const wrapper = mount(ProcessList, {
      props: {
        processes: mockProcesses,
        loading: false
      }
    })

    expect(wrapper.find('.empty').exists()).toBe(false)
    expect(wrapper.find('.loading').exists()).toBe(false)
  })

  describe('expansion toggle', () => {
    it('toggles expanded state on ProcessItem toggle event', async () => {
      const wrapper = mount(ProcessList, {
        props: {
          processes: mockProcesses,
          loading: false
        }
      })

      const items = wrapper.findAllComponents(ProcessItem)

      // Initially not expanded
      expect(items[0].props('expanded')).toBe(false)

      // Click to expand first item
      await items[0].vm.$emit('toggle')
      await wrapper.vm.$nextTick()

      expect(wrapper.findAllComponents(ProcessItem)[0].props('expanded')).toBe(true)
      expect(wrapper.findAllComponents(ProcessItem)[1].props('expanded')).toBe(false)
    })

    it('collapses expanded item when toggled again', async () => {
      const wrapper = mount(ProcessList, {
        props: {
          processes: mockProcesses,
          loading: false
        }
      })

      const items = wrapper.findAllComponents(ProcessItem)

      // Expand first item
      await items[0].vm.$emit('toggle')
      await wrapper.vm.$nextTick()

      // Collapse first item
      await items[0].vm.$emit('toggle')
      await wrapper.vm.$nextTick()

      expect(wrapper.findAllComponents(ProcessItem)[0].props('expanded')).toBe(false)
    })

    it('allows multiple items to be expanded simultaneously', async () => {
      const wrapper = mount(ProcessList, {
        props: {
          processes: mockProcesses,
          loading: false
        }
      })

      const items = wrapper.findAllComponents(ProcessItem)

      // Expand both items
      await items[0].vm.$emit('toggle')
      await items[1].vm.$emit('toggle')
      await wrapper.vm.$nextTick()

      expect(wrapper.findAllComponents(ProcessItem)[0].props('expanded')).toBe(true)
      expect(wrapper.findAllComponents(ProcessItem)[1].props('expanded')).toBe(true)
    })

    it('toggles expansion independently for each process by pid', async () => {
      const processes = [
        createMockProcess(100, 'process1.exe'),
        createMockProcess(200, 'process2.exe'),
        createMockProcess(300, 'process3.exe')
      ]

      const wrapper = mount(ProcessList, {
        props: {
          processes,
          loading: false
        }
      })

      const items = wrapper.findAllComponents(ProcessItem)

      // Expand first and third items
      await items[0].vm.$emit('toggle')
      await items[2].vm.$emit('toggle')
      await wrapper.vm.$nextTick()

      const updatedItems = wrapper.findAllComponents(ProcessItem)
      expect(updatedItems[0].props('expanded')).toBe(true)
      expect(updatedItems[1].props('expanded')).toBe(false)
      expect(updatedItems[2].props('expanded')).toBe(true)
    })
  })

  describe('event forwarding', () => {
    it('emits close event with pid when ProcessItem emits close', async () => {
      const wrapper = mount(ProcessList, {
        props: {
          processes: mockProcesses,
          loading: false
        }
      })

      const items = wrapper.findAllComponents(ProcessItem)
      await items[0].vm.$emit('close')

      expect(wrapper.emitted('close')).toHaveLength(1)
      expect(wrapper.emitted('close')![0]).toEqual([1234])
    })

    it('emits close for correct process when multiple exist', async () => {
      const wrapper = mount(ProcessList, {
        props: {
          processes: mockProcesses,
          loading: false
        }
      })

      const items = wrapper.findAllComponents(ProcessItem)

      // Close second process
      await items[1].vm.$emit('close')

      expect(wrapper.emitted('close')).toHaveLength(1)
      expect(wrapper.emitted('close')![0]).toEqual([5678])
    })

    it('emits addToAutoClose with process object', async () => {
      const wrapper = mount(ProcessList, {
        props: {
          processes: mockProcesses,
          loading: false
        }
      })

      const items = wrapper.findAllComponents(ProcessItem)
      await items[0].vm.$emit('addToAutoClose')

      expect(wrapper.emitted('addToAutoClose')).toHaveLength(1)
      expect(wrapper.emitted('addToAutoClose')![0]).toEqual([mockProcesses[0]])
    })

    it('emits permanentClose with process object', async () => {
      const wrapper = mount(ProcessList, {
        props: {
          processes: mockProcesses,
          loading: false
        }
      })

      const items = wrapper.findAllComponents(ProcessItem)
      await items[0].vm.$emit('permanentClose')

      expect(wrapper.emitted('permanentClose')).toHaveLength(1)
      expect(wrapper.emitted('permanentClose')![0]).toEqual([mockProcesses[0]])
    })

    it('emits correct process for addToAutoClose when multiple exist', async () => {
      const wrapper = mount(ProcessList, {
        props: {
          processes: mockProcesses,
          loading: false
        }
      })

      const items = wrapper.findAllComponents(ProcessItem)

      // Add second process to auto-close
      await items[1].vm.$emit('addToAutoClose')

      expect(wrapper.emitted('addToAutoClose')).toHaveLength(1)
      expect(wrapper.emitted('addToAutoClose')![0]).toEqual([mockProcesses[1]])
    })

    it('emits correct process for permanentClose when multiple exist', async () => {
      const wrapper = mount(ProcessList, {
        props: {
          processes: mockProcesses,
          loading: false
        }
      })

      const items = wrapper.findAllComponents(ProcessItem)

      // Permanently close second process
      await items[1].vm.$emit('permanentClose')

      expect(wrapper.emitted('permanentClose')).toHaveLength(1)
      expect(wrapper.emitted('permanentClose')![0]).toEqual([mockProcesses[1]])
    })
  })

  it('maintains expanded state when processes update', async () => {
    const wrapper = mount(ProcessList, {
      props: {
        processes: mockProcesses,
        loading: false
      }
    })

    // Expand first item
    await wrapper.findAllComponents(ProcessItem)[0].vm.$emit('toggle')

    // Update props with new processes array
    const updatedProcesses = [
      mockProcesses[0],
      createMockProcess(9999, 'new-process.exe')
    ]

    await wrapper.setProps({ processes: updatedProcesses })

    // First item should still be expanded
    expect(wrapper.findAllComponents(ProcessItem)[0].props('expanded')).toBe(true)
    expect(wrapper.findAllComponents(ProcessItem)[1].props('expanded')).toBe(false)
  })

  it('renders with single process', () => {
    const wrapper = mount(ProcessList, {
      props: {
        processes: [createMockProcess(1234, 'single.exe')],
        loading: false
      }
    })

    expect(wrapper.findAllComponents(ProcessItem)).toHaveLength(1)
  })

  it('handles processes with same pid but different data updates', async () => {
    const process = createMockProcess(1234, 'process.exe')
    const wrapper = mount(ProcessList, {
      props: {
        processes: [process],
        loading: false
      }
    })

    // Expand the item
    await wrapper.findComponent(ProcessItem).vm.$emit('toggle')

    const updatedProcess = { ...process, cpuUsage: 50.0 }
    await wrapper.setProps({ processes: [updatedProcess] })

    const item = wrapper.findComponent(ProcessItem)
    expect(item.props('expanded')).toBe(true)
    expect(item.props('process').cpuUsage).toBe(50.0)
  })
})
