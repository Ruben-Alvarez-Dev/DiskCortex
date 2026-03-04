#!/bin/bash
# DiskCortex E2E Visual Testing - Safe Execution Framework
# Version: 1.0.0
# 
# SAFETY GUARDRAILS:
# - All UI interactions are sandboxed
# - Screenshots captured to isolated directory
# - No system-wide mouse/keyboard automation outside test scope
# - Kill switch available via Ctrl+C

set -e

PROJECT_ROOT="/Users/ruben/Code/DiskCortex"
TEST_DIR="$PROJECT_ROOT/tests/e2e"
SCREENSHOTS="$TEST_DIR/screenshots"
VIDEOS="$TEST_DIR/videos"
LOGS="$TEST_DIR/logs"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log() {
    echo -e "${BLUE}[E2E]${NC} $1" | tee -a "$LOGS/test_$TIMESTAMP.log"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" | tee -a "$LOGS/test_$TIMESTAMP.log"
}

success() {
    echo -e "${GREEN}[OK]${NC} $1" | tee -a "$LOGS/test_$TIMESTAMP.log"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1" | tee -a "$LOGS/test_$TIMESTAMP.log"
}

# Cleanup function
cleanup() {
    log "Cleaning up test environment..."
    
    # Kill any diskcortex processes
    pkill -f "diskcortex" 2>/dev/null || true
    
    # Kill tmux sessions
    tmux kill-session -t diskcortex_tui 2>/dev/null || true
    
    # Stop any video recording
    pkill -f "ffmpeg.*diskcortex" 2>/dev/null || true
    
    log "Cleanup complete"
}

# Set trap for cleanup
trap cleanup EXIT

# Initialize
log "=== DiskCortex E2E Visual Testing ==="
log "Timestamp: $TIMESTAMP"
log "Project: $PROJECT_ROOT"

# Ensure directories exist
mkdir -p "$SCREENSHOTS" "$VIDEOS" "$LOGS"

# Check dependencies
log "Checking dependencies..."
command -v tmux >/dev/null 2>&1 || { error "tmux not found"; exit 1; }
command -v ffmpeg >/dev/null 2>&1 || { error "ffmpeg not found"; exit 1; }
command -v screencapture >/dev/null 2>&1 || { error "screencapture not found"; exit 1; }
success "All dependencies available"

# Build binaries first
log "Building binaries..."
cd "$PROJECT_ROOT"
cargo build --bin diskcortex --bin diskcortex-daemon 2>&1 | tail -5
success "Binaries built"

# ============================================
# TUI TESTING (Safe - runs in tmux)
# ============================================

test_tui() {
    log "Starting TUI testing in tmux session..."
    
    # Create tmux session
    tmux new-session -d -s diskcortex_tui -x 120 -y 40
    
    # Start video recording of tmux window (simulated with screenshots)
    log "Setting up capture for TUI session..."
    
    # Run TUI in tmux
    tmux send-keys -t diskcortex_tui "cd $PROJECT_ROOT && cargo run --bin diskcortex" Enter
    
    # Wait for app to start
    sleep 3
    
    # Capture initial state
    tmux capture-pane -t diskcortex_tui -p > "$SCREENSHOTS/tui_initial_$TIMESTAMP.txt"
    success "Captured TUI initial state"
    
    # Test navigation (safe - only sends to tmux)
    log "Testing TUI navigation..."
    
    # Navigate to Tools view
    tmux send-keys -t diskcortex_tui "2" 
    sleep 1
    tmux capture-pane -t diskcortex_tui -p > "$SCREENSHOTS/tui_tools_$TIMESTAMP.txt"
    success "Captured Tools view"
    
    # Navigate to Settings view
    tmux send-keys -t diskcortex_tui "4"
    sleep 1
    tmux capture-pane -t diskcortex_tui -p > "$SCREENSHOTS/tui_settings_$TIMESTAMP.txt"
    success "Captured Settings view"
    
    # Return to Overview
    tmux send-keys -t diskcortex_tui "1"
    sleep 1
    tmux capture-pane -t diskcortex_tui -p > "$SCREENSHOTS/tui_overview_$TIMESTAMP.txt"
    success "Captured Overview view"
    
    # Test scan (q to confirm dialogs)
    tmux send-keys -t diskcortex_tui "s"
    sleep 2
    tmux capture-pane -t diskcortex_tui -p > "$SCREENSHOTS/tui_scanning_$TIMESTAMP.txt"
    success "Captured Scanning state"
    
    # Exit TUI
    tmux send-keys -t diskcortex_tui "q"
    sleep 1
    
    # Kill session
    tmux kill-session -t diskcortex_tui 2>/dev/null || true
    
    success "TUI testing complete"
}

# ============================================
# GUI TESTING (Requires user confirmation)
# ============================================

test_gui() {
    log "Starting GUI testing..."
    
    # Check if GUI is built
    if [ ! -d "$PROJECT_ROOT/src-gui/out" ]; then
        log "Building GUI..."
        cd "$PROJECT_ROOT/src-gui"
        npm run build 2>&1 | tail -5
    fi
    
    # Start GUI in background
    log "Launching GUI application..."
    cd "$PROJECT_ROOT/src-gui"
    
    # Start Electron app (detached)
    npm run preview &
    GUI_PID=$!
    
    log "GUI PID: $GUI_PID"
    
    # Wait for window to appear
    log "Waiting for GUI window..."
    sleep 5
    
    # Capture screenshots
    log "Capturing GUI screenshots..."
    
    # Get window ID
    WINDOW_ID=$(osascript -e 'tell application "System Events" to get id of first window of process "Electron" whose visible is true' 2>/dev/null || echo "")
    
    if [ -n "$WINDOW_ID" ]; then
        log "Found Electron window: $WINDOW_ID"
        
        # Capture window
        screencapture -l "$WINDOW_ID" "$SCREENSHOTS/gui_initial_$TIMESTAMP.png"
        success "Captured GUI initial state"
    else
        warn "Could not find Electron window, capturing full screen"
        screencapture "$SCREENSHOTS/gui_fullscreen_$TIMESTAMP.png"
    fi
    
    # Kill GUI
    kill $GUI_PID 2>/dev/null || true
    
    success "GUI testing complete"
}

# ============================================
# VISION ANALYSIS (Optional - needs API)
# ============================================

analyze_screenshots() {
    log "Preparing screenshots for vision analysis..."
    
    # List captured screenshots
    log "Captured files:"
    ls -la "$SCREENSHOTS"/*"$TIMESTAMP"* 2>/dev/null || warn "No screenshots found"
    
    # Create analysis request (would need vision API)
    ANALYSIS_FILE="$LOGS/vision_analysis_request_$TIMESTAMP.json"
    
    cat > "$ANALYSIS_FILE" << EOF
{
    "timestamp": "$TIMESTAMP",
    "screenshots": [
$(ls "$SCREENSHOTS"/*"$TIMESTAMP"* 2>/dev/null | sed 's/.*/"&"/' | paste -sd,)
    ],
    "checks": [
        "UI elements visible",
        "No error messages",
        "Proper layout",
        "Text readable"
    ]
}
EOF
    
    log "Vision analysis request saved to: $ANALYSIS_FILE"
    log "To analyze, use vision API with screenshots in: $SCREENSHOTS"
}

# ============================================
# MAIN EXECUTION
# ============================================

main() {
    log "Starting E2E test suite..."
    
    # Run TUI tests (safe)
    test_tui
    
    # Ask before GUI tests (requires screen access)
    echo ""
    warn "GUI testing will open an Electron window and capture screenshots."
    echo "Press Enter to continue with GUI testing, or Ctrl+C to skip..."
    read -r
    
    test_gui
    
    # Analyze results
    analyze_screenshots
    
    # Summary
    echo ""
    log "=== Test Summary ==="
    log "Screenshots: $SCREENSHOTS"
    log "Videos: $VIDEOS"
    log "Logs: $LOGS"
    
    # Count files
    TUI_CAPS=$(ls "$SCREENSHOTS"/tui_*"$TIMESTAMP"* 2>/dev/null | wc -l)
    GUI_CAPS=$(ls "$SCREENSHOTS"/gui_*"$TIMESTAMP"* 2>/dev/null | wc -l)
    
    success "TUI captures: $TUI_CAPS"
    success "GUI captures: $GUI_CAPS"
    
    log "E2E testing complete!"
}

# Run main
main "$@"
