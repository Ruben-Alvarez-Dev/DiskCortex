// Cleanup options interface
export interface CleanupOptions {
  tools: string[]
  dryRun: boolean
  age?: number
  excludePaths?: string[]
}

export interface HealthStatus {
  status: 'ok' | 'error'
  version?: string
  uptime?: number
  message?: string
}

export interface ScanResult {
  tools: DetectedTool[]
  totalSize: number
  scanTime: number
  error?: string
}

export interface DetectedTool {
  id: string
  name: string
  category: string
  riskLevel: 'safe' | 'low' | 'medium' | 'high'
  size: number
  paths: string[]
  description: string
  lastUsed?: string
}

export interface CleanupResult {
  success: boolean
  cleanedTools: string[]
  freedSpace: number
  errors?: string[]
  duration: number
}
