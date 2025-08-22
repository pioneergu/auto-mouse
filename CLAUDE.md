# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Auto Mouse is a cross-platform "stay awake" application that prevents screen timeout by automatically moving the mouse cursor at configured intervals. It features a GUI built with egui/eframe and supports both Windows and Linux platforms.

## Build and Development Commands

```bash
# Build the project (debug)
cargo build

# Build optimized release version
cargo build --release

# Run the application (debug mode with console)
cargo run

# Run optimized release version
cargo run --release

# Check for compilation errors without building
cargo check

# Run tests (if any)
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Architecture Overview

The application follows a modular Rust architecture with four main modules:

### Core Modules
- **`gui/`** - GUI components using egui framework
  - `app.rs` - Main application window with collapsible UI, settings, controls, and statistics
  - `widgets.rs` - Reusable UI components (currently unused but available for future expansion)
- **`mouse/`** - Cross-platform mouse control functionality
  - `controller.rs` - Mouse movement logic with platform-specific implementations
- **`config/`** - Configuration management
  - `settings.rs` - Settings serialization/deserialization with JSON persistence
- **`timer/`** - Timer functionality for auto-stop feature
  - `simple_timer.rs` - Timer implementation for scheduled stops

### Key Dependencies
- **egui/eframe**: Cross-platform GUI framework (v0.26.2)
- **Platform-specific mouse control**:
  - Windows: `winapi` and `windows` crates for Windows API
  - Linux: `x11` and `libc` for X11 API
- **serde**: Serialization for settings persistence
- **tokio**: Async runtime (full features enabled)
- **chrono**: Date/time handling for statistics and timer
- **anyhow**: Error handling
- **dirs**: Cross-platform directory paths

### Application Features
- **Cross-platform support**: Windows and Linux
- **Collapsible UI**: Compact mode for minimal screen space usage
- **Timer functionality**: Auto-stop after specified duration
- **Statistics tracking**: Movement count and last activity time
- **Configurable settings**:
  - Movement interval (1-300 seconds)
  - Movement distance (1-100 pixels)
  - Timer duration (1-480 minutes)
  - Sound notifications (placeholder)
  - Start minimized option
- **Persistent configuration**: JSON-based settings storage

### Application Flow
1. Main entry point (`main.rs`) initializes logging and launches the GUI
2. `AutoMouseApp` manages application state, UI rendering, and handles startup minimization
3. `MouseController` runs mouse movement logic with platform-specific implementations
4. `SimpleTimer` manages auto-stop functionality
5. Settings are persisted to user config directory as JSON

### Platform-Specific Features

#### Windows
- Compiled without console window in release mode (`windows_subsystem = "windows"`)
- Uses Windows API for precise mouse control
- Settings stored in `%APPDATA%/auto-mouse/config.json`

#### Linux
- Uses X11 API for mouse control
- Settings stored in `~/.config/auto-mouse/config.json`

### UI Features
- **Korean language support**: Custom D2Coding font embedded
- **Collapsible interface**: Expandable/collapsible settings panel
- **Real-time status**: Shows current state, timer countdown, and last activity
- **Start minimized**: Optional startup in minimized state (controlled via settings)
- **Auto-resize**: Window automatically adjusts size based on collapsed state

### Recent Updates
- Added timer functionality for auto-stop feature
- Implemented start minimized functionality
- Cross-platform mouse control support
- Enhanced UI with collapsible design
- Added statistics tracking