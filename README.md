# MouseTool

A simple system tray tool written in Rust by a beginner developer. This tool helps keep your Steam account active by simulating mouse movements to prevent system sleep.


If you're learning Rust and want to understand this project, feel free to fork and experiment!


Credits go to https://cnb.cool/kori.dev/insomnia


## About This Project

This is a learning project for Rust beginners. It demonstrates basic concepts like:

- System tray integration
- Thread management
- Mouse control
- State management
- Error handling

## Features

- System tray icon with status indicators
- Manual enable/disable functionality
- Smart user activity detection
- Smooth mouse movement simulation
- Perfect for keeping Steam account active

## Installation

### Build from Source

```bash
cargo build --release
```

### Usage

1. Run the program and you'll see an icon in your system tray
2. Right-click the icon to:
   - Enable/disable auto-movement
   - Quit the program

## Status Indicators

- 🟢 Running
- 🟡 Idle
- 🔴 Disabled
