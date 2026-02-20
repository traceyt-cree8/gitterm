# GitTerm

A Git status viewer with integrated terminal, built with Iced.

![GitTerm](assets/icon.png)

## Features

- 🖥️ **Integrated Terminal** - Full-featured terminal emulator with scrollback
- 📊 **Git Integration** - Real-time git status, diffs, and file navigation
- 🌐 **HTTP Log Viewer** - Browse terminal output in your browser with perfect text selection
- 📁 **File Viewer** - View files with syntax highlighting, copy, and browser export
- 🎨 **Native UI** - macOS menu bar integration, theme toggle
- ⌨️ **Keyboard Shortcuts** - Vim-style navigation, Cmd+K to clear terminal
- 🔍 **Search** - Search through terminal scrollback (Cmd+F)

## Quick Start

### macOS

Download the latest release or build from source:

```bash
cargo build --release
./scripts/bundle.sh
open target/GitTerm.app
```

### Features Overview

#### HTTP Log Server (localhost:3030, optional)
- View all terminal logs in your browser
- Perfect text selection and copy
- Live search
- Auto-updates every 5 seconds
- Disabled by default; toggle from the app menu/UI when needed

#### File Viewer
- Click any file to preview
- "Copy All" button for instant clipboard copy
- "Open in Browser" for viewing with line numbers

#### Keyboard Shortcuts
- `Cmd+K` - Clear terminal
- `Cmd+F` - Search terminal output
- `Cmd+G` / `Cmd+Shift+G` - Next/previous search match
- `Cmd+1-9` - Switch tabs
- `Cmd++/-` - Increase/decrease terminal font
- `Cmd+Shift++/-` - Increase/decrease UI font
- `j/k` - Navigate files (when viewing diff)

## Building for Other Platforms

See [BUILD.md](BUILD.md) for detailed build instructions for Windows and Linux.

## Performance Notes

See [docs/PERFORMANCE_TUNING.md](docs/PERFORMANCE_TUNING.md) for:
- recent rendering/performance changes
- profiling guidance (`GITTERM_PERF=1`)
- all tuning knobs and recommended adjustment order

### Quick Cross-Platform Build (GitHub Actions)

1. Fork and push your `iced_term_fork` changes
2. Update `Cargo.toml` to use git dependency
3. Push to GitHub - CI builds for all platforms automatically

## Architecture

GitTerm is built on:
- **Iced** - Cross-platform GUI framework
- **iced_term** - Terminal emulator widget (custom fork)
- **git2** - Git integration
- **warp** - HTTP server for log viewer
- **muda** - Native menu bar (macOS)

## Development

```bash
# Run in development
cargo run

# Build release
cargo build --release

# Create macOS app bundle
./scripts/bundle.sh
```

## License

MIT

## Credits

- Based on [iced_term](https://github.com/Harzu/iced_term) by Harzu
- Built with ❤️ using Rust and Iced
