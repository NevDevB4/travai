# Travai Desktop App - Build Guide

## Prerequisites

### Windows Setup
1. **Install Rust**
   ```bash
   https://rustup.rs/
   ```

2. **Install Node.js and pnpm**
   ```bash
   # Node.js from https://nodejs.org/
   npm install -g pnpm
   ```

3. **Install LM Studio**
   - Download from https://lmstudio.ai/
   - Launch LM Studio and load CodeLlama 13B model
   - Ensure it's running on `localhost:1234`

## Development Setup

### 1. Install Dependencies
```bash
cd /path/to/ai-code-assistant-desktop
pnpm install
```

### 2. Run Development Server
```bash
pnpm tauri:dev
```

This will:
- Start the Vite dev server on http://localhost:5173
- Launch the Tauri window
- Enable hot module reloading

### 3. Test Features
- **Global Hotkey**: Press `Ctrl+Shift+T` to toggle the window
- **System Tray**: Right-click the system tray icon
- **Chat**: Type a message and press Ctrl+Enter or click Send
- **Code Generation**: Select "Generate" mode and describe the code
- **File Upload**: Click the upload button to analyze code files

## Building for Windows

### 1. Create Installer
```bash
pnpm tauri:build
```

This creates:
- `src-tauri/target/release/bundle/msi/Travai_1.0.0_x64_en-US.msi` (MSI installer)
- `src-tauri/target/release/bundle/nsis/Travai_1.0.0_x64-setup.exe` (NSIS installer)

### 2. Customize Build
Edit `tauri.conf.json` to customize:
- App name and version
- Window dimensions
- Bundle settings
- Icon paths

### 3. Code Signing (Optional)
For production releases, sign the executable:
```bash
# Generate certificate
certutil -genkey -f -exportable -len 2048 "C:\path\to\key.pvk"

# Configure in tauri.conf.json
"bundle": {
  "windows": {
    "certificateThumbprint": "YOUR_THUMBPRINT",
    "signingIdentity": "YOUR_IDENTITY"
  }
}
```

## Troubleshooting

### LM Studio Connection Issues
- Ensure LM Studio is running
- Check that CodeLlama 13B is loaded
- Verify API is accessible at `http://localhost:1234`
- Check Windows Firewall settings

### Hotkey Not Working
- Ensure Travai has proper permissions
- Try a different hotkey in `src-tauri/src/hotkey.rs`
- Check if another app is using the same hotkey

### Build Failures
```bash
# Clean build
cargo clean
pnpm tauri:build

# Check Rust version
rustc --version

# Update Rust
rustup update
```

## Performance Optimization

### Reduce App Size
- Remove unused dependencies from `Cargo.toml`
- Use `--release` flag for optimized builds
- Strip debug symbols: `strip target/release/travai.exe`

### Improve Startup Time
- Preload LLM model in LM Studio
- Cache conversations locally
- Lazy-load UI components

## Distribution

### Windows Store
1. Create Microsoft Partner Center account
2. Prepare app package
3. Submit for certification

### Direct Distribution
1. Host MSI/EXE on your server
2. Create update mechanism using Tauri updater
3. Provide installation instructions

## Configuration Files

### `tauri.conf.json`
Main Tauri configuration:
- Window settings
- Build options
- Bundle configuration
- Security settings

### `Cargo.toml`
Rust dependencies:
- Tauri framework
- LLM client
- Database driver
- System integration libraries

### `package.json`
Node.js dependencies:
- React and React DOM
- Tauri API bindings
- UI libraries (Lucide, Sonner)
- Build tools (Vite, TypeScript)

## Development Tips

1. **Hot Reload**: Changes to React files auto-reload without restart
2. **Debug Console**: Open DevTools with F12
3. **Logs**: Check console output for errors
4. **Database**: SQLite file stored in `%APPDATA%\Travai\travai.db`
5. **State**: Window state saved in `%APPDATA%\Travai\window_state.json`

## Next Steps

- [ ] Test on multiple Windows versions (10, 11)
- [ ] Create installer branding
- [ ] Set up auto-update mechanism
- [ ] Add crash reporting
- [ ] Implement analytics
- [ ] Create user documentation
