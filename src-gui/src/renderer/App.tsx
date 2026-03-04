import { useState, useEffect, useCallback } from 'react';
import { 
  Brain, 
  Scan, 
  Trash2, 
  Settings, 
  CheckCircle2,
  AlertTriangle,
  HardDrive,
  Package,
  Container,
  Terminal,
  RefreshCw,
  ChevronRight,
  Shield,
  Zap,
  Wifi,
  WifiOff,
  Loader2
} from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';
import './styles/App.css';
import { apiClient } from '../services/apiClient';
import { toolService, Tool as ServiceTool } from '../services/toolService';

// Types
interface Tool {
  id: string;
  name: string;
  category: string;
  size: number;
  risk: 'safe' | 'low' | 'medium' | 'high';
  description: string;
  installed: boolean;
  cache_paths: string[];
}

interface AppState {
  tools: Tool[];
  selectedTools: Set<string>;
  totalSize: number;
  scanning: boolean;
  view: 'overview' | 'tools' | 'cleanup' | 'settings';
  daemonConnected: boolean;
  loading: boolean;
  error: string | null;
}

// Utility functions
// Utility functions
function formatSize(bytes: number): string {
  const GB = 1024 * 1024 * 1024;
  const MB = 1024 * 1024;
  const KB = 1024;
  
  if (bytes >= GB) return `${(bytes / GB).toFixed(1)} GB`;
  if (bytes >= MB) return `${(bytes / MB).toFixed(1)} MB`;
  if (bytes >= KB) return `${(bytes / KB).toFixed(1)} KB`;
  return `${bytes} B`;
}

function getRiskColor(risk: string): string {
  switch (risk) {
    case 'safe': return 'text-green-400';
    case 'low': return 'text-yellow-400';
    case 'medium': return 'text-orange-400';
    case 'high': return 'text-red-400';
    default: return 'text-gray-400';
  }
}

function getRiskBg(risk: string): string {
  switch (risk) {
    case 'safe': return 'bg-green-500/20 border-green-500/30';
    case 'low': return 'bg-yellow-500/20 border-yellow-500/30';
    case 'medium': return 'bg-orange-500/20 border-orange-500/30';
    case 'high': return 'bg-red-500/20 border-red-500/30';
    default: return 'bg-gray-500/20 border-gray-500/30';
  }
}

function getCategoryIcon(category: string) {
  switch (category.toLowerCase()) {
    case 'ai_tool': return <Brain className="w-4 h-4" />;
    case 'ide': return <Terminal className="w-4 h-4" />;
    case 'package_manager': return <Package className="w-4 h-4" />;
    case 'container': return <Container className="w-4 h-4" />;
    default: return <HardDrive className="w-4 h-4" />;
  }
}

// Components
function Sidebar({ 
  view, 
  onViewChange,
  connected 
}: { 
  view: string; 
  onViewChange: (view: 'overview' | 'tools' | 'cleanup' | 'settings') => void;
  connected: boolean;
}) {
  const navItems = [
    { id: 'overview', label: 'Overview', icon: <Brain className="w-5 h-5" /> },
    { id: 'tools', label: 'Tools', icon: <HardDrive className="w-5 h-5" /> },
    { id: 'cleanup', label: 'Cleanup', icon: <Trash2 className="w-5 h-5" /> },
    { id: 'settings', label: 'Settings', icon: <Settings className="w-5 h-5" /> },
  ];

  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <Brain className="w-8 h-8 text-cyan-400" />
        <h1 className="text-xl font-bold bg-gradient-to-r from-cyan-400 to-purple-400 bg-clip-text text-transparent">
          DiskCortex
        </h1>
      </div>
      
      <nav className="sidebar-nav">
        {navItems.map((item) => (
          <button
            key={item.id}
            onClick={() => onViewChange(item.id as 'overview' | 'tools' | 'cleanup' | 'settings')}
            className={`nav-item ${view === item.id ? 'active' : ''}`}
          >
            {item.icon}
            <span>{item.label}</span>
            {view === item.id && <ChevronRight className="w-4 h-4 ml-auto" />}
          </button>
        ))}
      </nav>

      <div className="sidebar-footer">
        <div className={`status-indicator ${connected ? 'connected' : 'disconnected'}`}>
          {connected ? (
            <>
              <Wifi className="w-4 h-4" />
              <span>Daemon Connected</span>
            </>
          ) : (
            <>
              <WifiOff className="w-4 h-4" />
              <span>Daemon Offline</span>
            </>
          )}
        </div>
      </div>
    </aside>
  );
}

function ToolCard({ 
  tool, 
  selected, 
  onToggle 
}: { 
  tool: Tool; 
  selected: boolean;
  onToggle: () => void;
}) {
  return (
    <motion.div
      layout
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      className={`tool-card ${selected ? 'selected' : ''}`}
      onClick={onToggle}
    >
      <div className="tool-header">
        <div className="tool-icon">
          {getCategoryIcon(tool.category)}
        </div>
        <div className="tool-info">
          <h3 className="tool-name">{tool.name}</h3>
          <p className="tool-description">{tool.description}</p>
        </div>
        <div className={`checkbox ${selected ? 'checked' : ''}`}>
          {selected && <CheckCircle2 className="w-5 h-5" />}
        </div>
      </div>
      
      <div className="tool-footer">
        <span className="tool-size">{formatSize(tool.size)}</span>
        <span className={`risk-badge ${getRiskBg(tool.risk)} ${getRiskColor(tool.risk)}`}>
          {tool.risk.toUpperCase()}
        </span>
      </div>
    </motion.div>
  );
}

function LoadingState() {
  return (
    <div className="loading-state">
      <Loader2 className="w-8 h-8 animate-spin text-cyan-400" />
      <span>Cargando datos del daemon...</span>
    </div>
  );
}

function ErrorState({ error, onRetry }: { error: string; onRetry: () => void }) {
  return (
    <div className="error-state">
      <AlertTriangle className="w-12 h-12 text-red-400 mb-4" />
      <h3>Error de conexión</h3>
      <p>{error}</p>
      <button onClick={onRetry} className="btn-primary mt-4">
        <RefreshCw className="w-4 h-4" />
        Reintentar
      </button>
    </div>
  );
}

function OverviewView({ state, onScan }: { state: AppState; onScan: () => void }) {
  const selectedSize = state.tools
    .filter(t => state.selectedTools.has(t.id))
    .reduce((acc, t) => acc + t.size, 0);

  if (state.loading) return <LoadingState />;
  if (state.error) return <ErrorState error={state.error} onRetry={onScan} />;

  return (
    <div className="view overview-view">
      <header className="view-header">
        <h2>System Overview</h2>
        <button 
          onClick={onScan}
          disabled={state.scanning || !state.daemonConnected}
          className="btn-primary"
        >
          {state.scanning ? (
            <>
              <RefreshCw className="w-4 h-4 animate-spin" />
              Scanning...
            </>
          ) : (
            <>
              <Scan className="w-4 h-4" />
              Scan System
            </>
          )}
        </button>
      </header>

      <div className="stats-grid">
        <div className="stat-card">
          <div className="stat-icon cyan">
            <HardDrive className="w-6 h-6" />
          </div>
          <div className="stat-info">
            <span className="stat-value">{formatSize(state.totalSize)}</span>
            <span className="stat-label">Total Cache Size</span>
          </div>
        </div>

        <div className="stat-card">
          <div className="stat-icon purple">
            <Package className="w-6 h-6" />
          </div>
          <div className="stat-info">
            <span className="stat-value">{state.tools.length}</span>
            <span className="stat-label">Tools Detected</span>
          </div>
        </div>

        <div className="stat-card">
          <div className="stat-icon green">
            <CheckCircle2 className="w-6 h-6" />
          </div>
          <div className="stat-info">
            <span className="stat-value">{state.selectedTools.size}</span>
            <span className="stat-label">Selected for Cleanup</span>
          </div>
        </div>

        <div className="stat-card">
          <div className="stat-icon orange">
            <Zap className="w-6 h-6" />
          </div>
          <div className="stat-info">
            <span className="stat-value">{formatSize(selectedSize)}</span>
            <span className="stat-label">Space to Reclaim</span>
          </div>
        </div>
      </div>

      <section className="tools-section">
        <h3>Top Space Consumers</h3>
        <div className="tools-list">
          <AnimatePresence>
            {state.tools.slice(0, 5).map((tool) => (
              <ToolCard
                key={tool.id}
                tool={tool}
                selected={state.selectedTools.has(tool.id)}
                onToggle={() => {}}
              />
            ))}
          </AnimatePresence>
        </div>
      </section>
    </div>
  );
}

function ToolsView({ state, onToggleTool }: { state: AppState; onToggleTool: (id: string) => void }) {
  const [filter, setFilter] = useState<string>('all');

  const categories = ['all', ...new Set(state.tools.map(t => t.category))];
  
  const filteredTools = filter === 'all' 
    ? state.tools 
    : state.tools.filter(t => t.category === filter);

  if (state.loading) return <LoadingState />;
  if (state.error) return <ErrorState error={state.error} onRetry={() => window.location.reload()} />;

  return (
    <div className="view tools-view">
      <header className="view-header">
        <h2>Installed Tools</h2>
        <div className="filter-tabs">
          {categories.map((cat) => (
            <button
              key={cat}
              onClick={() => setFilter(cat)}
              className={`filter-tab ${filter === cat ? 'active' : ''}`}
            >
              {cat.replace('_', ' ').toUpperCase()}
            </button>
          ))}
        </div>
      </header>

      <div className="tools-grid">
        <AnimatePresence>
          {filteredTools.map((tool) => (
            <ToolCard
              key={tool.id}
              tool={tool}
              selected={state.selectedTools.has(tool.id)}
              onToggle={() => onToggleTool(tool.id)}
            />
          ))}
        </AnimatePresence>
      </div>
    </div>
  );
}

function CleanupView({ state, onCleanup }: { state: AppState; onCleanup: () => void }) {
  const selectedTools = state.tools.filter(t => state.selectedTools.has(t.id));
  const totalToClean = selectedTools.reduce((acc, t) => acc + t.size, 0);

  if (state.loading) return <LoadingState />;
  if (state.error) return <ErrorState error={state.error} onRetry={() => window.location.reload()} />;

  return (
    <div className="view cleanup-view">
      <header className="view-header">
        <h2>Cleanup Plan</h2>
        <button 
          onClick={onCleanup}
          disabled={selectedTools.length === 0 || !state.daemonConnected}
          className="btn-danger"
        >
          <Trash2 className="w-4 h-4" />
          Clean {selectedTools.length} Tools
        </button>
      </header>

      {selectedTools.length === 0 ? (
        <div className="empty-state">
          <AlertTriangle className="w-16 h-16 text-yellow-400 mb-4" />
          <h3>No Tools Selected</h3>
          <p>Select tools from the Tools view to add them to the cleanup plan.</p>
        </div>
      ) : (
        <>
          <div className="cleanup-summary">
            <div className="summary-item">
              <span className="summary-label">Tools to Clean</span>
              <span className="summary-value">{selectedTools.length}</span>
            </div>
            <div className="summary-item">
              <span className="summary-label">Space to Reclaim</span>
              <span className="summary-value text-green-400">{formatSize(totalToClean)}</span>
            </div>
          </div>

          <div className="tools-list">
            <AnimatePresence>
              {selectedTools.map((tool) => (
                <ToolCard
                  key={tool.id}
                  tool={tool}
                  selected={true}
                  onToggle={() => {}}
                />
              ))}
            </AnimatePresence>
          </div>

          <div className="cleanup-warning">
            <Shield className="w-5 h-5 text-yellow-400" />
            <span>High-risk items will require additional confirmation</span>
          </div>
        </>
      )}
    </div>
  );
}

function SettingsView() {
  return (
    <div className="view settings-view">
      <header className="view-header">
        <h2>Settings</h2>
      </header>

      <div className="settings-sections">
        <section className="settings-section">
          <h3>General</h3>
          <div className="setting-item">
            <div className="setting-info">
              <span className="setting-label">Auto-scan on startup</span>
              <span className="setting-description">
                Automatically scan for tools when DiskCortex starts
              </span>
            </div>
            <label className="toggle">
              <input type="checkbox" defaultChecked />
              <span className="toggle-slider" />
            </label>
          </div>
          <div className="setting-item">
            <div className="setting-info">
              <span className="setting-label">Confirm before cleanup</span>
              <span className="setting-description">
                Show confirmation dialog before executing cleanup
              </span>
            </div>
            <label className="toggle">
              <input type="checkbox" defaultChecked />
              <span className="toggle-slider" />
            </label>
          </div>
        </section>

        <section className="settings-section">
          <h3>Safety</h3>
          <div className="setting-item">
            <div className="setting-info">
              <span className="setting-label">Skip high-risk items</span>
              <span className="setting-description">
                Automatically exclude high-risk items from bulk cleanup
              </span>
            </div>
            <label className="toggle">
              <input type="checkbox" defaultChecked />
              <span className="toggle-slider" />
            </label>
          </div>
          <div className="setting-item">
            <div className="setting-info">
              <span className="setting-label">Use trash instead of delete</span>
              <span className="setting-description">
                Move files to trash instead of permanently deleting
              </span>
            </div>
            <label className="toggle">
              <input type="checkbox" />
              <span className="toggle-slider" />
            </label>
          </div>
        </section>

        <section className="settings-section">
          <h3>Scheduling</h3>
          <div className="setting-item">
            <div className="setting-info">
              <span className="setting-label">Cleanup Schedule</span>
              <span className="setting-description">
                How often to run automatic cleanup
              </span>
            </div>
            <select className="select">
              <option value="disabled">Disabled</option>
              <option value="daily">Daily</option>
              <option value="weekly">Weekly</option>
              <option value="monthly">Monthly</option>
            </select>
          </div>
        </section>
      </div>
    </div>
  );
}

// Helper: Convert ServiceTool to local Tool type
function mapServiceToolToLocal(tool: ServiceTool): Tool {
  return {
    id: tool.id,
    name: tool.name,
    category: tool.category,
    size: tool.size || 0,
    risk: tool.risk || 'low',
    description: tool.description || '',
    installed: tool.installed,
    cache_paths: tool.cachePaths || [],
  };
}

// Main App - Conectado a daemon API real (SIN mock data)
export default function App() {
  const [state, setState] = useState<AppState>({
    tools: [],
    selectedTools: new Set(),
    totalSize: 0,
    scanning: false,
    view: 'overview',
    daemonConnected: false,
    loading: true,
    error: null,
  });

  // Cargar herramientas desde el daemon API
  const loadToolsFromDaemon = useCallback(async () => {
    setState(prev => ({ ...prev, loading: true, error: null }));
    
    try {
      // Verificar conexión con daemon
      const isConnected = await apiClient.isConnected();
      
      if (!isConnected) {
        setState(prev => ({
          ...prev,
          loading: false,
          daemonConnected: false,
          error: 'No se puede conectar con el daemon de DiskCortex. Asegúrate de que esté ejecutándose en localhost:7331',
        }));
        return;
      }

      // Cargar herramientas desde el API
      const tools = await toolService.getToolsWithUsage();
      const mappedTools = tools.map(mapServiceToolToLocal);
      const totalSize = mappedTools.reduce((acc, t) => acc + t.size, 0);

      setState(prev => ({
        ...prev,
        tools: mappedTools,
        totalSize,
        daemonConnected: true,
        loading: false,
        error: null,
      }));
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Error desconocido al cargar datos';
      setState(prev => ({
        ...prev,
        loading: false,
        daemonConnected: false,
        error: errorMessage,
      }));
    }
  }, []);

  // Cargar datos al montar el componente
  useEffect(() => {
    loadToolsFromDaemon();
  }, [loadToolsFromDaemon]);

  // Ejecutar scan real
  const handleScan = async () => {
    setState(prev => ({ ...prev, scanning: true }));
    
    try {
      // Iniciar discovery scan
      const { jobId } = await toolService.startDiscovery();
      
      // Polling para verificar estado del scan
      let attempts = 0;
      const maxAttempts = 30; // 30 segundos máximo
      
      while (attempts < maxAttempts) {
        const status = await toolService.getDiscoveryStatus(jobId);
        
        if (status.status === 'completed') {
          // Recargar herramientas después del scan
          await loadToolsFromDaemon();
          setState(prev => ({ ...prev, scanning: false }));
          return;
        }
        
        if (status.status === 'failed') {
          throw new Error(status.error || 'El scan falló');
        }
        
        // Esperar 1 segundo antes del siguiente check
        await new Promise(resolve => setTimeout(resolve, 1000));
        attempts++;
      }
      
      throw new Error('Timeout: El scan tardó demasiado');
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Error durante el scan';
      setState(prev => ({ 
        ...prev, 
        scanning: false,
        error: errorMessage 
      }));
    }
  };

  const handleToggleTool = (id: string) => {
    setState(prev => {
      const newSelected = new Set(prev.selectedTools);
      if (newSelected.has(id)) {
        newSelected.delete(id);
      } else {
        newSelected.add(id);
      }
      return { ...prev, selectedTools: newSelected };
    });
  };

  const handleCleanup = async () => {
    // TODO: Implementar cleanup real vía API
    alert('Cleanup ejecutado - Implementar llamada a API /cleanup/execute');
  };

  return (
    <div className="app">
      <Sidebar 
        view={state.view} 
        onViewChange={(view) => setState(prev => ({ ...prev, view }))} 
        connected={state.daemonConnected}
      />
      
      <main className="main-content">
        <AnimatePresence mode="wait">
          {state.view === 'overview' && (
            <OverviewView key="overview" state={state} onScan={handleScan} />
          )}
          {state.view === 'tools' && (
            <ToolsView key="tools" state={state} onToggleTool={handleToggleTool} />
          )}
          {state.view === 'cleanup' && (
            <CleanupView key="cleanup" state={state} onCleanup={handleCleanup} />
          )}
          {state.view === 'settings' && (
            <SettingsView key="settings" />
          )}
        </AnimatePresence>
      </main>
    </div>
  );
}
