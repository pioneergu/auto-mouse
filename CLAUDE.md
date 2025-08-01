# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Auto Mouse is a Windows-based "stay awake" application that prevents screen timeout by automatically moving the mouse cursor at configured intervals. It features a GUI built with egui/eframe and uses Windows API for mouse control.

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

The application follows a modular Rust architecture with three main modules:

### Core Modules
- **`gui/`** - GUI components using egui framework
  - `app.rs` - Main application window with collapsible UI, settings, controls, and statistics
  - `widgets.rs` - Reusable UI components
- **`mouse/`** - Mouse control functionality
  - `controller.rs` - Mouse movement logic using Windows API (winapi crate)
- **`config/`** - Configuration management
  - `settings.rs` - Settings serialization/deserialization with JSON persistence

### Key Dependencies
- **egui/eframe**: Cross-platform GUI framework
- **winapi**: Windows API bindings for mouse control
- **serde**: Serialization for settings persistence
- **tokio**: Async runtime (full features enabled)
- **chrono**: Date/time handling for statistics

### Application Flow
1. Main entry point (`main.rs`) initializes logging and launches the GUI
2. `AutoMouseApp` manages application state and UI rendering
3. `MouseController` runs in a separate thread, performing periodic mouse movements
4. Settings are persisted to user config directory as JSON

### Windows-Specific Features
- Compiled without console window in release mode (`windows_subsystem = "windows"`)
- Uses Windows API for precise mouse control
- Custom D2Coding font embedded for Korean text support

### Configuration
- Settings stored in `%APPDATA%/auto-mouse/config.json`
- Configurable interval (1-300 seconds) and movement distance (1-100 pixels)
- Sound notifications and startup minimization options