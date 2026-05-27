import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { mount } from '@vue/test-utils'
import SearchBar from '../SearchBar.vue'

describe('SearchBar', () => {
  beforeEach(() => {
    vi.useFakeTimers()
  })

  afterEach(() => {
    vi.useRealTimers()
  })

  it('renders search input and refresh button', () => {
    const wrapper = mount(SearchBar, {
      props: { modelValue: '' }
    })

    expect(wrapper.find('.search-input').exists()).toBe(true)
    expect(wrapper.find('.btn-refresh').exists()).toBe(true)
    expect(wrapper.find('.search-icon').exists()).toBe(true)
    expect(wrapper.find('.search-icon').text()).toBe('🔍')
  })

  it('displays correct placeholder text', () => {
    const wrapper = mount(SearchBar, {
      props: { modelValue: '' }
    })

    const input = wrapper.find('.search-input')
    expect(input.attributes('placeholder')).toBe('搜索进程...')
  })

  it('emits update:modelValue after 300ms debounce', async () => {
    const wrapper = mount(SearchBar, {
      props: { modelValue: '' }
    })

    const input = wrapper.find('.search-input')
    await input.setValue('test search')

    // Should not emit immediately
    expect(wrapper.emitted('update:modelValue')).toBeUndefined()

    // Advance timer by less than 300ms
    vi.advanceTimersByTime(200)
    expect(wrapper.emitted('update:modelValue')).toBeUndefined()

    // Advance to 300ms
    vi.advanceTimersByTime(100)
    expect(wrapper.emitted('update:modelValue')).toHaveLength(1)
    expect(wrapper.emitted('update:modelValue')![0]).toEqual(['test search'])
  })

  it('emits update:modelValue with debounce after multiple input changes', async () => {
    const wrapper = mount(SearchBar, {
      props: { modelValue: '' }
    })

    const input = wrapper.find('.search-input')

    // Multiple rapid changes
    await input.setValue('a')
    vi.advanceTimersByTime(100)
    await input.setValue('ab')
    vi.advanceTimersByTime(100)
    await input.setValue('abc')

    // Should not emit yet
    expect(wrapper.emitted('update:modelValue')).toBeUndefined()

    // Advance full debounce time from last input
    vi.advanceTimersByTime(300)
    expect(wrapper.emitted('update:modelValue')).toHaveLength(1)
    expect(wrapper.emitted('update:modelValue')![0]).toEqual(['abc'])
  })

  it('emits refresh when refresh button clicked', async () => {
    const wrapper = mount(SearchBar, {
      props: { modelValue: '' }
    })

    await wrapper.find('.btn-refresh').trigger('click')

    expect(wrapper.emitted('refresh')).toHaveLength(1)
  })

  it('refresh button displays correct label', () => {
    const wrapper = mount(SearchBar, {
      props: { modelValue: '' }
    })

    expect(wrapper.find('.btn-refresh').text()).toBe('刷新')
  })

  it('syncs localValue with modelValue prop changes', async () => {
    const wrapper = mount(SearchBar, {
      props: { modelValue: 'initial' }
    })

    expect(wrapper.find('.search-input').element.value).toBe('initial')

    await wrapper.setProps({ modelValue: 'updated' })

    expect(wrapper.find('.search-input').element.value).toBe('updated')
  })

  it('maintains local input value while typing before debounce', async () => {
    const wrapper = mount(SearchBar, {
      props: { modelValue: '' }
    })

    const input = wrapper.find('.search-input')
    await input.setValue('typing...')

    expect(input.element.value).toBe('typing...')
  })

  it('clears debounce timer on unmount', async () => {
    const wrapper = mount(SearchBar, {
      props: { modelValue: '' }
    })

    const input = wrapper.find('.search-input')
    await input.setValue('test')

    wrapper.unmount()

    // Should not throw when advancing timers after unmount
    expect(() => vi.advanceTimersByTime(300)).not.toThrow()
  })
})
