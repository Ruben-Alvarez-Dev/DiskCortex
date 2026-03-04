/**
 * Tool Service
 * 
 * Servicio para gestión de herramientas detectadas por DiskCortex.
 * Conecta con la API del daemon para obtener datos REALES.
 * 
 * @module services/toolService
 */

import apiClient, { ApiError, Pagination } from './apiClient';

// ─────────────────────────────────────────────────────────────────────────────
// Types (basados en OpenAPI spec)
// ─────────────────────────────────────────────────────────────────────────────

export type ToolCategory = 
  | 'ai_tool' 
  | 'ide' 
  | 'build_system' 
  | 'package_manager' 
  | 'runtime' 
  | 'container' 
  | 'other';

export type RiskLevel = 'safe' | 'low' | 'medium' | 'high';

export interface Tool {
  id: string;
  name: string;
  category: ToolCategory;
  description: string;
  version?: string;
  installed: boolean;
  installPath?: string;
  cachePaths: string[];
  configPaths: string[];
  safeToClean: string[];
  requiresConfirmation: boolean;
  metadata?: Record<string, unknown>;
  // Computed fields for UI
  size?: number;
  risk?: RiskLevel;
}

export interface ToolListResponse {
  data: Tool[];
  pagination: Pagination;
}

export interface ToolDiskUsage {
  toolId: string;
  toolName: string;
  totalSize: number;
  cacheSize: number;
  configSize: number;
  paths: Array<{
    path: string;
    size: number;
    type: 'cache' | 'config' | 'install';
  }>;
}

export interface ScanResult {
  jobId: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  progress: number;
  toolsFound: number;
  totalSize: number;
  errors?: string[];
}

export interface DiscoveryJobStatus {
  jobId: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  progress: number;
  toolsDiscovered: number;
  startedAt: string;
  completedAt?: string;
  error?: string;
}

// ─────────────────────────────────────────────────────────────────────────────
// Tool Service Class
// ─────────────────────────────────────────────────────────────────────────────

class ToolService {
  private static instance: ToolService;

  private constructor() {}

  /**
   * Get singleton instance
   */
  public static getInstance(): ToolService {
    if (!ToolService.instance) {
      ToolService.instance = new ToolService();
    }
    return ToolService.instance;
  }

  /**
   * Get all registered tools
   */
  public async getTools(category?: ToolCategory): Promise<Tool[]> {
    const params = category ? { category } : {};
    const response = await apiClient.get<ToolListResponse>('/registry/tools', { params });
    return response.data;
  }

  /**
   * Get tool by ID
   */
  public async getTool(toolId: string): Promise<Tool> {
    return apiClient.get<Tool>(`/registry/tools/${toolId}`);
  }

  /**
   * Get disk usage for a specific tool
   */
  public async getToolDiskUsage(toolId: string): Promise<ToolDiskUsage> {
    return apiClient.get<ToolDiskUsage>(`/scans/tools/${toolId}/usage`);
  }

  /**
   * Get disk usage for all tools
   */
  public async getAllToolsDiskUsage(): Promise<ToolDiskUsage[]> {
    return apiClient.get<ToolDiskUsage[]>('/scans/tools/usage');
  }

  /**
   * Start discovery scan
   */
  public async startDiscovery(): Promise<{ jobId: string }> {
    return apiClient.post<{ jobId: string }>('/discovery/scan');
  }

  /**
   * Get discovery job status
   */
  public async getDiscoveryStatus(jobId: string): Promise<DiscoveryJobStatus> {
    return apiClient.get<DiscoveryJobStatus>(`/discovery/jobs/${jobId}`);
  }

  /**
   * Start full disk scan
   */
  public async startFullScan(): Promise<{ jobId: string }> {
    return apiClient.post<{ jobId: string }>('/scans/full');
  }

  /**
   * Get scan result
   */
  public async getScanResult(jobId: string): Promise<ScanResult> {
    return apiClient.get<ScanResult>(`/scans/${jobId}`);
  }

  /**
   * Enrich tools with disk usage data
   */
  public async getToolsWithUsage(): Promise<Tool[]> {
    const [tools, usageData] = await Promise.all([
      this.getTools(),
      this.getAllToolsDiskUsage().catch(() => []), // Graceful fallback
    ]);

    // Map usage data to tools
    const usageMap = new Map(
      usageData.map((usage) => [usage.toolId, usage])
    );

    return tools.map((tool) => {
      const usage = usageMap.get(tool.id);
      if (usage) {
        return {
          ...tool,
          size: usage.totalSize,
          risk: this.calculateRiskLevel(tool, usage),
        };
      }
      return tool;
    });
  }

  /**
   * Calculate risk level based on tool properties and usage
   */
  private calculateRiskLevel(tool: Tool, usage: ToolDiskUsage): RiskLevel {
    if (!tool.requiresConfirmation && tool.safeToClean.length > 0) {
      return 'safe';
    }
    if (tool.configPaths.length > 0 && usage.configSize > 0) {
      return 'high';
    }
    if (tool.requiresConfirmation) {
      return 'medium';
    }
    return 'low';
  }

  /**
   * Format bytes to human readable string
   */
  public formatSize(bytes: number): string {
    const GB = 1024 * 1024 * 1024;
    const MB = 1024 * 1024;
    const KB = 1024;

    if (bytes >= GB) return `${(bytes / GB).toFixed(1)} GB`;
    if (bytes >= MB) return `${(bytes / MB).toFixed(1)} MB`;
    if (bytes >= KB) return `${(bytes / KB).toFixed(1)} KB`;
    return `${bytes} B`;
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Export singleton instance
// ─────────────────────────────────────────────────────────────────────────────

export const toolService = ToolService.getInstance();
export default toolService;
