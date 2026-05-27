import { config } from '@vue/test-utils'
import { vi } from 'vitest'

// Mock Tauri APIs
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}))

// Mock crypto.randomUUID for configStore tests
Object.defineProperty(globalThis, 'crypto', {
  value: {
    randomUUID: vi.fn(() => 'test-uuid-1234-5678')
  }
})

config.global.stubs = {}
