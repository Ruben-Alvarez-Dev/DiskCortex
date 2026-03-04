#!/bin/bash
# DiskCortex GUI E2E Test - Automated (no prompts)
set -e

PROJECT_ROOT="/Users/ruben/Code/DiskCortex"
SCREENSHOTS="$PROJECT_ROOT/tests/e2e/screenshots"
LOGS="$PROJECT_ROOT/tests/e2e/logs"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

log() { echo "[GUI-TEST] $1" | tee -a "$LOGS/gui_test_$TIMESTAMP.log"; }

cleanup() {
    pkill -f "Electron.*DiskCortex" 2>/dev/null || true
    pkill -f "electron-vite" 2>/dev/null || true
}
trap cleanup EXIT

log "=== Starting GUI E2E Test ==="

# Build GUI if needed
cd "$PROJECT_ROOT/src-gui"
if [ ! -d "out" ]; then
    log "Building GUI..."
    npm run build 2>&1 | tail -10
fi

# Start GUI in dev mode (faster, has hot reload)
log "Launching GUI..."
npm run dev &
DEV_PID=$!
log "Dev server PID: $DEV_PID"

# Wait for Electron window
log "Waiting for Electron window (15s)..."
sleep 15

# Try to find and capture the window
log "Capturing screenshots..."

# Method 1: Use AppleScript to find Electron window
WINDOW_INFO=$(osascript -e '
tell application "System Events"
    set windowList to {}
    repeat with proc in (processes whose name contains "Electron")
        try
            set win to window 1 of proc
            set winTitle to name of win
            set winId to id of win
            set end of windowList to winId & ":" & winTitle
        end try
    end repeat
    return windowList as string
end tell
' 2>/dev/null || echo "")

if [ -n "$WINDOW_INFO" ]; then
    log "Found window: $WINDOW_INFO"
    
    # Extract window ID
    WIN_ID=$(echo "$WINDOW_INFO" | cut -d':' -f1)
    
    # Capture specific window
    screencapture -l "$WIN_ID" "$SCREENSHOTS/gui_initial_$TIMESTAMP.png" 2>/dev/null && \
        log "✅ Captured GUI window" || \
        log "⚠️ Failed to capture specific window, trying fullscreen"
fi

# Fallback: Full screen capture
screencapture "$SCREENSHOTS/gui_fullscreen_$TIMESTAMP.png" && \
    log "✅ Captured fullscreen"

# Kill dev server
kill $DEV_PID 2>/dev/null || true

log "=== GUI Test Complete ==="
log "Screenshots saved to: $SCREENSHOTS"
ls -la "$SCREENSHOTS"/*gui*"$TIMESTAMP"* 2>/dev/null || log "No GUI screenshots found"
