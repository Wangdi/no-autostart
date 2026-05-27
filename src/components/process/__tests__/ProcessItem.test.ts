import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import ProcessItem from '../ProcessItem.vue'
import { StartupType, RiskLevel, RecommendedAction } from '@/types'
import type { ProcessInfo } from '@/types'

describe('ProcessItem', () => {
  const createMockProcess = (overrides: Partial<ProcessInfo> = {}): ProcessInfo => ({
    pid: 1234,
    name: 'test-process.exe',
    executablePath: 'C:\\Program Files\\Test\\test-process.exe',
    publisher: 'Test Publisher',
    cpuUsage: 5.5,
    memoryUsage: 102400000, // ~97.6 MB
    runningTime: 3665, // 1h 1m 5s
    startupType: StartupType.RegistryRun,
    startupLocation: 'HKLM\\Run',
    isKnownProcess: true,
    riskLevel: RiskLevel.Safe,
    canClose: true,
    recommendedAction: RecommendedAction.CanClose,
    ...overrides
  })

  it('renders process row with correct data', () => {
    const process = createMockProcess()
    const wrapper = mount(ProcessItem, {
      props: {
        process,
        expanded: false
      }
    })

    expect(wrapper.find('.process-name').text()).toBe('test-process.exe')
    expect(wrapper.find('.process-pid').text()).toBe('1234')
    expect(wrapper.find('.process-cpu').text()).toBe('5.5%')
    expect(wrapper.find('.process-startup').text()).toBe('注册表启动')
  })

  it('shows expand icon as triangle right when not expanded', () => {
    const wrapper = mount(ProcessItem, {
      props: {
        process: createMockProcess(),
        expanded: false
      }
    })

    expect(wrapper.find('.expand-icon').text()).toBe('▶')
  })

  it('shows expand icon as triangle down when expanded', () => {
    const wrapper = mount(ProcessItem, {
      props: {
        process: createMockProcess(),
        expanded: true
      }
    })

    expect(wrapper.find('.expand-icon').text()).toBe('▼')
  })

  it('shows close button when canClose is true', () => {
    const wrapper = mount(ProcessItem, {
      props: {
        process: createMockProcess({ canClose: true }),
        expanded: false
      }
    })

    expect(wrapper.find('.btn-close').exists()).toBe(true)
    expect(wrapper.find('.btn-close').text()).toBe('×')
    expect(wrapper.find('.action-disabled').exists()).toBe(false)
  })

  it('shows disabled action when canClose is false', () => {
    const wrapper = mount(ProcessItem, {
      props: {
        process: createMockProcess({ canClose: false }),
        expanded: false
      }
    })

    expect(wrapper.find('.btn-close').exists()).toBe(false)
    expect(wrapper.find('.action-disabled').exists()).toBe(true)
    expect(wrapper.find('.action-disabled').text()).toBe('-')
  })

  it('emits toggle on row click', async () => {
    const wrapper = mount(ProcessItem, {
      props: {
        process: createMockProcess(),
        expanded: false
      }
    })

    await wrapper.find('.process-row').trigger('click')

    expect(wrapper.emitted('toggle')).toHaveLength(1)
  })

  it('emits close on close button click', async () => {
    const wrapper = mount(ProcessItem, {
      props: {
        process: createMockProcess({ canClose: true }),
        expanded: false
      }
    })

    await wrapper.find('.btn-close').trigger('click')

    expect(wrapper.emitted('close')).toHaveLength(1)
  })

  it('stops propagation when clicking close button', async () => {
    const wrapper = mount(ProcessItem, {
      props: {
        process: createMockProcess({ canClose: true }),
        expanded: false
      }
    })

    const closeButton = wrapper.find('.btn-close')
    await closeButton.trigger('click')

    // Should emit close but not toggle (due to @click.stop)
    expect(wrapper.emitted('close')).toHaveLength(1)
    expect(wrapper.emitted('toggle')).toBeUndefined()
  })

  it('shows details when expanded', () => {
    const wrapper = mount(ProcessItem, {
      props: {
        process: createMockProcess(),
        expanded: true
      }
    })

    expect(wrapper.find('.process-detail').exists()).toBe(true)
    expect(wrapper.text()).toContain('基本信息')
    expect(wrapper.text()).toContain('风险等级')
    expect(wrapper.text()).toContain('操作')
  })

  it('hides details when not expanded', () => {
    const wrapper = mount(ProcessItem, {
      props: {
        process: createMockProcess(),
        expanded: false
      }
    })

    expect(wrapper.find('.process-detail').exists()).toBe(false)
  })

  it('displays process details correctly when expanded', () => {
    const process = createMockProcess({
      name: 'my-app.exe',
      pid: 9876,
      executablePath: 'C:\\Apps\\my-app.exe',
      publisher: 'My Company',
      runningTime: 3665
    })

    const wrapper = mount(ProcessItem, {
      props: {
        process,
        expanded: true
      }
    })

    expect(wrapper.text()).toContain('进程名称:')
    expect(wrapper.text()).toContain('my-app.exe')
    expect(wrapper.text()).toContain('进程ID:')
    expect(wrapper.text()).toContain('9876')
    expect(wrapper.text()).toContain('可执行路径:')
    expect(wrapper.text()).toContain('C:\\Apps\\my-app.exe')
    expect(wrapper.text()).toContain('发布者:')
    expect(wrapper.text()).toContain('My Company')
    expect(wrapper.text()).toContain('运行时长:')
  })

  it('displays unknown publisher when publisher is undefined', () => {
    const process = createMockProcess({ publisher: undefined })

    const wrapper = mount(ProcessItem, {
      props: {
        process,
        expanded: true
      }
    })

    expect(wrapper.text()).toContain('未知')
  })

  it('displays risk badge with correct color and label', () => {
    const wrapper = mount(ProcessItem, {
      props: {
        process: createMockProcess({
          riskLevel: RiskLevel.Warning
        }),
        expanded: true
      }
    })

    const badge = wrapper.find('.risk-badge')
    expect(badge.text()).toBe('警告')
    expect(badge.attributes('style')).toContain('var(--color-danger)')
  })

  it('displays all risk level badges correctly', () => {
    const riskLevels = [
      { level: RiskLevel.Safe, label: '安全', color: 'var(--color-success)' },
      { level: RiskLevel.Caution, label: '谨慎', color: 'var(--color-warning)' },
      { level: RiskLevel.Warning, label: '警告', color: 'var(--color-danger)' },
      { level: RiskLevel.Unknown, label: '未知', color: 'var(--color-text-secondary)' }
    ]

    riskLevels.forEach(({ level, label, color }) => {
      const wrapper = mount(ProcessItem, {
        props: {
          process: createMockProcess({ riskLevel: level }),
          expanded: true
        }
      })

      const badge = wrapper.find('.risk-badge')
      expect(badge.text()).toBe(label)
      expect(badge.attributes('style')).toContain(color)
    })
  })

  it('emits close action from detail buttons', async () => {
    const wrapper = mount(ProcessItem, {
      props: {
        process: createMockProcess(),
        expanded: true
      }
    })

    const buttons = wrapper.findAll('.btn-action')
    await buttons[0].trigger('click') // 关闭进程

    expect(wrapper.emitted('close')).toHaveLength(1)
  })

  it('emits addToAutoClose action from detail buttons', async () => {
    const wrapper = mount(ProcessItem, {
      props: {
        process: createMockProcess(),
        expanded: true
      }
    })

    const buttons = wrapper.findAll('.btn-action')
    await buttons[1].trigger('click') // 加入自动关闭列表

    expect(wrapper.emitted('addToAutoClose')).toHaveLength(1)
  })

  it('emits permanentClose action from detail buttons', async () => {
    const wrapper = mount(ProcessItem, {
      props: {
        process: createMockProcess(),
        expanded: true
      }
    })

    const buttons = wrapper.findAll('.btn-action')
    await buttons[2].trigger('click') // 永久关闭

    expect(wrapper.emitted('permanentClose')).toHaveLength(1)
  })

  describe('formatMemory', () => {
    it('formats memory >= 1 MB correctly', () => {
      const wrapper = mount(ProcessItem, {
        props: {
          process: createMockProcess({ memoryUsage: 102400000 }), // ~97.66 MB
          expanded: false
        }
      })

      // 102400000 / (1024 * 1024) = 97.65625 MB
      expect(wrapper.find('.process-memory').text()).toBe('97.7 MB')
    })

    it('formats memory < 1 MB as KB', () => {
      const wrapper = mount(ProcessItem, {
        props: {
          process: createMockProcess({ memoryUsage: 512000 }), // 500 KB
          expanded: false
        }
      })

      // 512000 / 1024 = 500 KB
      expect(wrapper.find('.process-memory').text()).toBe('500 KB')
    })

    it('formats memory at 1 MB boundary correctly', () => {
      const wrapper = mount(ProcessItem, {
        props: {
          process: createMockProcess({ memoryUsage: 1048576 }), // Exactly 1 MB
          expanded: false
        }
      })

      expect(wrapper.find('.process-memory').text()).toBe('1.0 MB')
    })

    it('formats small memory values correctly', () => {
      const wrapper = mount(ProcessItem, {
        props: {
          process: createMockProcess({ memoryUsage: 1024 }), // 1 KB
          expanded: false
        }
      })

      expect(wrapper.find('.process-memory').text()).toBe('1 KB')
    })
  })

  describe('formatDuration', () => {
    it('formats duration with hours correctly', () => {
      const wrapper = mount(ProcessItem, {
        props: {
          process: createMockProcess({ runningTime: 3665 }), // 1h 1m 5s
          expanded: true
        }
      })

      expect(wrapper.text()).toContain('1小时1分')
    })

    it('formats duration with only minutes correctly', () => {
      const wrapper = mount(ProcessItem, {
        props: {
          process: createMockProcess({ runningTime: 185 }), // 3m 5s
          expanded: true
        }
      })

      expect(wrapper.text()).toContain('3分')
    })

    it('formats duration with only seconds correctly', () => {
      const wrapper = mount(ProcessItem, {
        props: {
          process: createMockProcess({ runningTime: 45 }), // 45s
          expanded: true
        }
      })

      expect(wrapper.text()).toContain('45秒')
    })

    it('formats zero seconds correctly', () => {
      const wrapper = mount(ProcessItem, {
        props: {
          process: createMockProcess({ runningTime: 0 }),
          expanded: true
        }
      })

      expect(wrapper.text()).toContain('0秒')
    })

    it('formats exactly 60 seconds as 1 minute', () => {
      const wrapper = mount(ProcessItem, {
        props: {
          process: createMockProcess({ runningTime: 60 }),
          expanded: true
        }
      })

      expect(wrapper.text()).toContain('1分')
    })

    it('formats exactly 3600 seconds as 1 hour', () => {
      const wrapper = mount(ProcessItem, {
        props: {
          process: createMockProcess({ runningTime: 3600 }),
          expanded: true
        }
      })

      expect(wrapper.text()).toContain('1小时0分')
    })
  })

  it('has correct CSS class when expanded', () => {
    const wrapper = mount(ProcessItem, {
      props: {
        process: createMockProcess(),
        expanded: true
      }
    })

    expect(wrapper.find('.process-item').classes()).toContain('expanded')
  })

  it('does not have expanded class when not expanded', () => {
    const wrapper = mount(ProcessItem, {
      props: {
        process: createMockProcess(),
        expanded: false
      }
    })

    expect(wrapper.find('.process-item').classes()).not.toContain('expanded')
  })
})
