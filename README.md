# RustKey Input Monitor

A lightweight, detailed input event monitoring tool written in Rust that captures and displays keyboard, mouse, and other input device events in Linux systems. This tool uses the libinput library to monitor input devices in real-time with a beautiful, colorful terminal interface.

## Features

- **Device Detection**: Shows when input devices are connected or disconnected
- **Keyboard Monitoring**: Displays key presses and releases with human-readable key names
- **Mouse Tracking**: Tracks and displays:
  - Mouse position coordinates
  - Mouse movement (delta values)
  - Button clicks with human-readable button names
  - Scroll wheel activity
- **Touch Events**: Detects and displays touch-based input
- **Gesture Recognition**: Identifies gesture-based interactions 
- **Tablet Support**: Works with drawing tablets and stylus devices
- **Beautiful UI**: Colorful terminal output with clear, visually distinct event types
- **Statistics**: Keeps track of total key presses and mouse clicks

## Prerequisites

- Linux operating system
- Rust programming language (with Cargo)
- libinput development package
- Administrative/sudo privileges

### System Dependencies

For Arch Linux:
```
sudo pacman -S libinput
```

For Ubuntu/Debian:
```
sudo apt install libinput-dev
```

For Fedora:
```
sudo dnf install libinput-devel
```

## Building the Project

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/rustkey.git
   cd rustkey/libinput_project
   ```

2. Build the project using Cargo:
   ```
   cargo build
   ```

## Running the Application

Since this application needs to access input devices, it requires administrative privileges:

```
sudo ./target/debug/libinput_project
```

For a release (optimized) build:
```
cargo build --release
sudo ./target/release/libinput_project
```

## Usage

Once running, the application will display a beautiful, colorful interface showing input events in real-time:

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

To exit the application, press `Ctrl+C`.

## Understanding the Output

- **Device Events**:
  - `➕ Device Added` - A new input device was connected (green)
  - `➖ Device Removed` - An input device was disconnected (red)

- **Keyboard Events**:
  - `⌨️  KEY PRESS DETECTED --> [ key name ] <-- (code: key_code)` - Shows which key was pressed (yellow/magenta)
  - `🔠 YOU PRESSED: [ key name ]` - Clear indicator of the pressed key with key press counter (green)
  - `⌨️  KEY RELEASE DETECTED --> [ key name ] <-- (code: key_code)` - Shows when a key is released (blue)

- **Mouse Events**:
  - `🖱️  Mouse motion - Position: (x, y), Delta: (dx, dy)` - Shows relative mouse movement with position (cyan)
  - `🖱️  Mouse absolute position: (x, y)` - Shows absolute mouse position (cyan)
  - `🖱️  Mouse button [button name] (code) - PRESSED at position: (x, y)` - Shows mouse button activity with click counter (magenta)
  - `🖱️  Scroll wheel: horizontal: [value], vertical: [value]` - Shows scroll wheel activity (cyan)

- **Other Events**:
  - `👆 Touch Event: [type]` - Touch screen/pad events (magenta)
  - `🤲 Gesture Event: [type]` - Multi-touch gesture events (magenta)
  - `✏️ Tablet Event` - Drawing tablet events (yellow)
  - `🔄 Switch Event` - Switch/toggle events (yellow)

## Recent Modifications

The most recent updates enhance the tool with:

1. Beautiful colorful terminal output with ANSI color codes
2. Stats tracking for key presses and mouse clicks
3. Improved user interface with clear visual grouping of event types
4. Comprehensive mouse position tracking (both absolute and relative)
5. Improved key name mapping with broader keyboard support
6. Detailed mouse button identification (LEFT, RIGHT, MIDDLE, etc.)
7. Better scroll wheel event reporting
8. Enhanced touch and gesture event information

## Setting Up a GitHub Repository

To share your project on GitHub:

1. Create a new repository on GitHub
   - Go to https://github.com/new
   - Name your repository (e.g., "rustkey")
   - Add a description: "A beautiful input event monitoring tool for Linux"
   - Choose public or private visibility
   - Click "Create repository"

2. Initialize your local Git repository (if not already done):
   ```
   cd libinput_project
   git init
   git add .
   git commit -m "Initial commit: RustKey Input Monitor"
   ```

3. Link your local repository to GitHub:
   ```
   git remote add origin https://github.com/yourusername/rustkey.git
   git branch -M main
   git push -u origin main
   ```

4. Update the repository with future changes:
   ```
   git add .
   git commit -m "Description of your changes"
   git push
   ```

## Troubleshooting

If you encounter permission issues:
- Make sure you're running with sudo
- Check that your user has access to input devices
- Verify that libinput is properly installed

If devices aren't detected:
- Check that your devices are recognized by the system (`lsusb`, `xinput list`)
- Ensure udev is properly configured
- Try reconnecting the devices

If colors don't display correctly:
- Make sure your terminal supports ANSI color codes
- Try a different terminal emulator if colors appear broken

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- The libinput team for their excellent input handling library
- The Rust community for the robust ecosystem 