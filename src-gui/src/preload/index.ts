import { contextBridge, ipcRenderer } from 'electron'

// Expose protected methods to renderer
const electronAPI = {
  // Window controls
  minimizeWindow: () => ipcRenderer.send('window:minimize'),
  maximizeWindow: () => ipcRenderer.send('window:maximize'),
  closeWindow: () => ipcRenderer.send('window:close'),

  // Daemon communication
  checkDaemonHealth: () => ipcRenderer.invoke('daemon:health'),
  scanTools: () => ipcRenderer.invoke('daemon:scan'),
  executeCleanup: (options: CleanupOptions) => ipcRenderer.invoke('daemon:cleanup', options),

  // Platform info
  platform: process.platform,
}

export interface CleanupOptions {
  tools: string[]
  dryRun: boolean
  age?: number
  excludePaths?: string[]
}

// Use `contextBridge` APIs to expose Electron APIs to renderer
if (process.contextIsolated) {
  try {
    contextBridge.exposeInMainWorld('electronAPI', electronAPI)
  } catch (error) {
    console.error('Failed to expose API:', error)
  }
} else {
  // @ts-ignore
  window.electronAPI = electronAPI
}
