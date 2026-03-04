import { create } from 'zustand'
import { DetectedTool, HealthStatus } from '../types'

interface AppState {
  // Connection
  daemonHealth: HealthStatus | null
  setDaemonHealth: (health: HealthStatus | null) => void

  // Tools
  detectedTools: DetectedTool[]
  setDetectedTools: (tools: DetectedTool[]) => void
  selectedTools: Set<string>
  toggleTool: (toolId: string) => void
  selectAll: () => void
  deselectAll: () => void

  // Cleanup options
  dryRun: boolean
  setDryRun: (dryRun: boolean) => void
  ageFilter: number | null
  setAgeFilter: (age: number | null) => void

  // UI state
  currentView: 'overview' | 'tools' | 'cleanup' | 'settings'
  setView: (view: 'overview' | 'tools' | 'cleanup' | 'settings') => void
  isScanning: boolean
  setIsScanning: (scanning: boolean) => void
  isCleaning: boolean
  setIsCleaning: (cleaning: boolean) => void

  // Computed
  totalSize: () => number
  selectedSize: () => number
}

export const useAppStore = create<AppState>((set, get) => ({
  // Connection
  daemonHealth: null,
  setDaemonHealth: (health) => set({ daemonHealth: health }),

  // Tools
  detectedTools: [],
  setDetectedTools: (tools) => set({ detectedTools: tools }),
  selectedTools: new Set<string>(),
  toggleTool: (toolId) => {
    const { selectedTools } = get()
    const newSet = new Set(selectedTools)
    if (newSet.has(toolId)) {
      newSet.delete(toolId)
    } else {
      newSet.add(toolId)
    }
    set({ selectedTools: newSet })
  },
  selectAll: () => {
    const { detectedTools } = get()
    set({ selectedTools: new Set(detectedTools.map(t => t.id)) })
  },
  deselectAll: () => set({ selectedTools: new Set() }),

  // Cleanup options
  dryRun: true,
  setDryRun: (dryRun) => set({ dryRun }),
  ageFilter: null,
  setAgeFilter: (age) => set({ ageFilter: age }),

  // UI state
  currentView: 'overview',
  setView: (view) => set({ currentView: view }),
  isScanning: false,
  setIsScanning: (scanning) => set({ isScanning: scanning }),
  isCleaning: false,
  setIsCleaning: (cleaning) => set({ isCleaning: cleaning }),

  // Computed
  totalSize: () => get().detectedTools.reduce((sum, t) => sum + t.size, 0),
  selectedSize: () => {
    const { detectedTools, selectedTools } = get()
    return detectedTools
      .filter(t => selectedTools.has(t.id))
      .reduce((sum, t) => sum + t.size, 0)
  },
}))
