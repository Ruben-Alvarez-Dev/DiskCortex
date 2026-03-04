import { app, BrowserWindow, ipcMain, shell } from 'electron'
import { join } from 'path'
import { electronApp, optimizer, is } from '@electron-toolkit/utils'

let mainWindow: BrowserWindow | null = null
const DAEMON_PORT = 7331
const DAEMON_URL = `http://127.0.0.1:${DAEMON_PORT}`

function createWindow(): void {
  mainWindow = new BrowserWindow({
    width: 1400,
    height: 900,
    minWidth: 1200,
    minHeight: 700,
    show: false,
    autoHideMenuBar: true,
    frame: process.platform === 'darwin' ? true : false,
    titleBarStyle: 'hiddenInset',
    trafficLightPosition: { x: 15, y: 15 },
    backgroundColor: '#0f172a',
    webPreferences: {
      preload: join(__dirname, '../preload/index.js'),
      sandbox: false,
      contextIsolation: true,
      nodeIntegration: false
    }
  })

  mainWindow.on('ready-to-show', () => {
    mainWindow?.show()
  })

  mainWindow.webContents.setWindowOpenHandler((details) => {
    shell.openExternal(details.url)
    return { action: 'deny' }
  })

  // HMR for renderer
  if (is.dev && process.env['ELECTRON_RENDERER_URL']) {
    mainWindow.loadURL(process.env['ELECTRON_RENDERER_URL'])
  } else {
    mainWindow.loadFile(join(__dirname, '../renderer/index.html'))
  }
}

// IPC handlers for window controls
ipcMain.on('window:minimize', () => {
  mainWindow?.minimize()
})

ipcMain.on('window:maximize', () => {
  if (mainWindow?.isMaximized()) {
    mainWindow.unmaximize()
  } else {
    mainWindow?.maximize()
  }
})

ipcMain.on('window:close', () => {
  mainWindow?.close()
})

// IPC for daemon communication
ipcMain.handle('daemon:health', async () => {
  try {
    const response = await fetch(`${DAEMON_URL}/health`)
    return await response.json()
  } catch {
    return { status: 'error', message: 'Daemon not running' }
  }
})

ipcMain.handle('daemon:scan', async () => {
  try {
    const response = await fetch(`${DAEMON_URL}/api/v1/tools/scan`)
    return await response.json()
  } catch {
    return { error: 'Failed to scan tools' }
  }
})

ipcMain.handle('daemon:cleanup', async (_event, options) => {
  try {
    const response = await fetch(`${DAEMON_URL}/api/v1/cleanup/execute`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(options)
    })
    return await response.json()
  } catch {
    return { error: 'Failed to execute cleanup' }
  }
})

// App lifecycle
app.whenReady().then(() => {
  electronApp.setAppUserModelId('com.diskcortex')

  app.on('browser-window-created', (_, window) => {
    optimizer.watchWindowShortcuts(window)
  })

  createWindow()

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) createWindow()
  })
})

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') app.quit()
})
