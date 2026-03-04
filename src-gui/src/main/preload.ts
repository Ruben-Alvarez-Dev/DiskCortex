import { contextBridge, ipcRenderer } from 'electron';

// Expose protected methods that allow the renderer process to use
// the ipcRenderer without exposing the entire object
contextBridge.exposeInMainWorld('electronAPI', {
  // Daemon
  getDaemonUrl: () => ipcRenderer.invoke('daemon:url'),
  checkDaemonStatus: () => ipcRenderer.invoke('daemon:status'),
  
  // Shell
  openExternal: (url: string) => ipcRenderer.invoke('shell:openExternal', url),
  
  // Platform info
  platform: process.platform,
  
  // App info
  getVersion: () => ipcRenderer.invoke('app:version'),
});

// Type definition for the exposed API
export interface ElectronAPI {
  getDaemonUrl: () => Promise<string>;
  checkDaemonStatus: () => Promise<boolean>;
  openExternal: (url: string) => Promise<void>;
  platform: NodeJS.Platform;
  getVersion: () => Promise<string>;
}

declare global {
  interface Window {
    electronAPI: ElectronAPI;
  }
}
