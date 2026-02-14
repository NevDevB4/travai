# Travai - AI Code Assistant for Windows

A native Windows desktop application powered by CodeLlama 13B running locally via LM Studio. Travai integrates seamlessly with Windows through global hotkeys, system tray, and clipboard monitoring.

## Features

### 🎯 Core Capabilities
- **Code Generation**: Generate code from natural language descriptions
- **Code Explanation**: Understand complex code with AI analysis
- **Debugging**: Get help identifying and fixing bugs
- **Code Optimization**: Improve performance and readability
- **File Analysis**: Upload and analyze entire code files
- **Conversation History**: Persistent local storage of all conversations

### 🪟 Windows Integration
- **Global Hotkey** (`Ctrl+Shift+T`): Activate Travai from anywhere
- **System Tray**: Quick access menu in Windows taskbar
- **Clipboard Integration**: Detect and analyze code from clipboard
- **Always-on-Top**: Optional floating window mode
- **Keyboard Shortcuts**: Full keyboard navigation support

### 🎨 Design
- **Brutalist Aesthetic**: Bold typography with Space Mono font
- **High Contrast**: Black on white for maximum readability
- **Responsive Layout**: Works on any screen size
- **Syntax Highlighting**: Beautiful code rendering with 25+ language support

### 🔒 Privacy
- **100% Local**: All processing happens on your machine
- **No Cloud**: CodeLlama 13B runs locally via LM Studio
- **Offline**: Works without internet connection
- **Data Privacy**: Conversations stored locally in SQLite

## Quick Start

### Prerequisites
- Windows 10 or 11
- [Rust](https://rustup.rs/)
- [Node.js](https://nodejs.org/) with pnpm
- [LM Studio](https://lmstudio.ai/) with CodeLlama 13B loaded

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd ai-code-assistant-desktop
   ```

2. **Install dependencies**
   ```bash
   pnpm install
   ```

3. **Start LM Studio**
   - Launch LM Studio
   - Load CodeLlama 13B model
   - Ensure API is running on `localhost:1234`

4. **Run development version**
   ```bash
   pnpm tauri:dev
   ```

5. **Build installer** (optional)
   ```bash
   pnpm tauri:build
   ```

## Usage

### Keyboard Shortcuts
| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+T` | Toggle Travai window (global) |
| `Ctrl+Enter` | Send message |
| `Escape` | Close dialogs |
| `Ctrl+N` | New conversation |
| `Ctrl+L` | Focus message input |

### Chat Modes
- **Chat**: General conversation about code
- **Generate**: Create code from descriptions
- **Explain**: Understand existing code
- **Debug**: Find and fix issues
- **Optimize**: Improve code quality

### File Upload
1. Click the upload button (📁)
2. Drag and drop a code file or select from disk
3. Optionally ask a specific question
4. Click "ANALYZE FILE"
5. Get instant AI analysis

## Architecture

### Frontend
- **React 19** with TypeScript
- **Tauri** for desktop integration
- **Vite** for fast development
- **Tailwind CSS** with custom brutalist theme

### Backend
- **Rust** with Tauri framework
- **SQLite** for local database
- **LM Studio API** for CodeLlama 13B
- **System integration** (hotkeys, tray, clipboard)

### Storage
- **Conversations**: `%APPDATA%\Travai\travai.db`
- **Window State**: `%APPDATA%\Travai\window_state.json`
- **Logs**: `%APPDATA%\Travai\logs\`

## Configuration

### LM Studio Setup
1. Download [LM Studio](https://lmstudio.ai/)
2. Search for "CodeLlama 13B"
3. Download the model
4. Click "Load Model"
5. Ensure local server is running on port 1234

### Customization
Edit `tauri.conf.json` to customize:
- Window size and position
- App name and version
- Bundle settings
- Security policies

## Troubleshooting

### LM Studio Connection Failed
```
Error: Cannot connect to LM Studio. Make sure it's running on localhost:1234
```
**Solution**: 
- Verify LM Studio is running
- Check that CodeLlama 13B is loaded
- Ensure port 1234 is not blocked by firewall

### Hotkey Not Working
**Solution**:
- Check if another app is using `Ctrl+Shift+T`
- Restart Travai
- Check Windows Defender/Antivirus permissions

### Slow Responses
**Solution**:
- Ensure LM Studio is using GPU acceleration
- Close other applications
- Reduce model context window in LM Studio settings

## Development

### Project Structure
```
ai-code-assistant-desktop/
├── src/                    # React frontend
│   ├── components/        # React components
│   ├── styles/           # CSS files
│   ├── App.tsx           # Main app component
│   └── main.tsx          # Entry point
├── src-tauri/            # Rust backend
│   ├── src/
│   │   ├── main.rs       # Tauri app setup
│   │   ├── commands.rs   # Tauri commands
│   │   ├── db.rs         # Database operations
│   │   ├── llm.rs        # LM Studio client
│   │   ├── hotkey.rs     # Global hotkey handler
│   │   ├── tray.rs       # System tray
│   │   └── clipboard.rs  # Clipboard integration
│   └── Cargo.toml        # Rust dependencies
├── tauri.conf.json       # Tauri configuration
├── package.json          # Node dependencies
└── BUILD_GUIDE.md        # Detailed build instructions
```

### Build Commands
```bash
# Development
pnpm tauri:dev

# Production build
pnpm tauri:build

# Clean build
cargo clean && pnpm tauri:build

# Run tests
pnpm test
```

## Performance

### System Requirements
- **Minimum**: 8GB RAM, Intel i5 or equivalent
- **Recommended**: 16GB RAM, RTX 3060 or better GPU
- **Optimal**: 32GB RAM, RTX 4070 or better

### Response Times
- First response: ~5-10 seconds (model loading)
- Subsequent responses: ~2-5 seconds
- File analysis: ~3-8 seconds depending on file size

## Security

- ✅ No telemetry or tracking
- ✅ No internet connection required
- ✅ All data stored locally
- ✅ Open source (inspect code)
- ✅ No login or authentication required

## License

MIT License - See LICENSE file for details

## Support

For issues, questions, or feature requests:
1. Check [BUILD_GUIDE.md](BUILD_GUIDE.md) for detailed setup
2. Review troubleshooting section above
3. Check LM Studio documentation
4. Create an issue on GitHub

## Roadmap

- [ ] Auto-update mechanism
- [ ] Multiple model support
- [ ] Code snippet sharing
- [ ] Conversation export (PDF/Markdown)
- [ ] Custom keyboard shortcuts
- [ ] Dark mode theme
- [ ] Plugin system
- [ ] Remote LM Studio support

## Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## Acknowledgments

- [Tauri](https://tauri.app/) - Desktop framework
- [LM Studio](https://lmstudio.ai/) - Local LLM hosting
- [CodeLlama](https://github.com/facebookresearch/codellama) - AI model
- [React](https://react.dev/) - UI framework
- [Rust](https://www.rust-lang.org/) - Systems language
