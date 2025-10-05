# RustKey Input Monitor

A real-time input event visualization tool built in Rust that provides comprehensive monitoring of keyboard, mouse, and touch events with detailed statistics and colorful terminal output.

## Features

### Core Functionality
- **Real-time Input Monitoring**: Captures and displays keyboard, mouse, touch, and gesture events as they occur
- **Cross-Device Support**: Works with keyboards, mice, touchpads, touchscreens, and drawing tablets
- **Event Statistics**: Tracks key press counts, mouse click counts, and device connection events
- **High-Performance Processing**: Optimized event loop with minimal CPU overhead

### Technical Capabilities
- **System-Level Integration**: Uses Linux libinput library for low-level input device access
- **Comprehensive Key Mapping**: Supports 100+ key codes including function keys, modifiers, numpad, and media keys
- **Mouse Position Tracking**: Real-time absolute and relative mouse position monitoring
- **Scroll Event Detection**: Captures horizontal and vertical scroll events with precise values
- **Device Hot-Plugging**: Automatically detects when input devices are connected or disconnected

### User Experience
- **Color-Coded Output**: ANSI color coding for different event types (keyboard, mouse, touch, etc.)
- **Detailed Event Information**: Shows key codes, button mappings, coordinates, and timing
- **Professional Terminal Interface**: Clean, organized display with proper formatting
- **Real-Time Feedback**: Instant visualization of all input events

## Technical Architecture

### Dependencies
- **libinput**: Linux input handling library for device abstraction
- **evdev**: Linux event device interface for low-level input access
- **Rust Standard Library**: Core functionality for file handling, threading, and I/O

### Key Components
- **Event Loop**: Non-blocking event processing with 5ms polling interval
- **Device Interface**: Custom libinput interface implementation for device access
- **State Management**: Mouse position tracking and statistics accumulation
- **Color System**: ANSI escape sequences for terminal output formatting

## System Requirements

### Supported Platforms
- **Linux**: Arch Linux, Ubuntu/Debian, Fedora, and other distributions
- **Permissions**: Requires root access (sudo) for input device access
- **Dependencies**: libinput development libraries

### Installation Requirements

#### Arch Linux
```bash
sudo pacman -S libinput
```

#### Ubuntu/Debian
```bash
sudo apt install libinput-dev
```

#### Fedora
```bash
sudo dnf install libinput-devel
```

## Installation & Usage

### Quick Start
```bash
# Clone the repository
git clone https://github.com/Nivesh12345/rustkey.git
cd rustkey

# Build the project
cargo build

# Run with elevated privileges
sudo ./target/debug/libinput_project
```

### Production Build
```bash
# Optimized release build
cargo build --release
sudo ./target/release/libinput_project
```

## Output Example

```
══════════════════════════════════════════════════════
           🎮 RUSTKEY INPUT MONITOR 🎮           
══════════════════════════════════════════════════════

A beautiful way to visualize your input events in real-time

Waiting for input events... (press Ctrl+C to exit)

------------------------------------------
➕ Device Added
➕ Device Added
🖱️  Mouse absolute position: (1254.23, 876.49)
⌨️  KEY PRESS DETECTED --> A <-- (code: 30)
🔠 YOU PRESSED: [ A ] (Total key presses: 1)
⌨️  KEY RELEASE DETECTED --> A <-- (code: 30)
🖱️  Mouse button LEFT (272) - PRESSED at position: (1254.23, 876.49) (Total clicks: 1)
```

## Event Types Supported

### Keyboard Events
- Key press/release detection
- Comprehensive key code mapping (letters, numbers, function keys, modifiers)
- Special key support (arrow keys, numpad, media keys)
- Real-time key press statistics

### Mouse Events
- Absolute and relative position tracking
- Button press/release detection (left, right, middle, side buttons)
- Scroll wheel events (horizontal and vertical)
- Click count statistics

### Touch & Gesture Events
- Touch screen input detection
- Gesture recognition
- Multi-touch support

### Device Management
- Hot-plug detection
- Device connection/disconnection events
- Multiple device support

## Development

- **Event Processing**: Custom libinput interface with proper error handling
- **Memory Management**: Efficient state tracking with minimal allocations
- **Performance**: Optimized polling loop with configurable sleep intervals
- **Error Handling**: Robust error management for device access and system calls

## Troubleshooting

### Common Issues

**Permission Denied Errors**
```bash
# Ensure you're running with elevated privileges
sudo ./target/rebug/libinput_project
```

**Input Devices Not Detected**
- Verify libinput is properly installed
- Check device permissions in `/dev/input/`
- Try unplugging and reconnecting devices

**Terminal Display Issues**
- Ensure your terminal supports ANSI color codes
- Try different terminal emulators (GNOME Terminal, Konsole, etc.)

## Future Enhancements

### Planned Features
- **Configuration System**: Customizable color schemes and event filtering
- **Logging Capabilities**: Save event data to files for analysis
- **Performance Metrics**: Detailed statistics and usage analytics
- **GUI Interface**: Optional graphical user interface
- **Cross-Platform Support**: Windows and macOS compatibility

### Technical Improvements
- **Async/Await**: Modern async event handling
- **Plugin System**: Extensible event processing architecture
- **Performance Profiling**: Built-in performance monitoring
- **Unit Testing**: Comprehensive test coverage


---

*Built with Rust for high-performance input event monitoring and visualization*