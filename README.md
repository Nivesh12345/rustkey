# 🎮 RustKey Input Monitor

Hey everyone! I'm excited to share my first Rust project - a colorful input event monitor that shows your keyboard and mouse activity in real-time! 

I built this while learning Rust and wanted to understand how our computers detect input events. It was super fun to make, and I thought others might find it useful or interesting too!

<!-- TODO: Add a screenshot of the program running! -->
<!-- ![RustKey Demo](https://i.imgur.com/your-image-here.jpg) -->

## ✨ What This Project Does

This tool shows you in real-time what's happening when you:
- Type on your keyboard
- Move your mouse
- Click buttons
- Use your touchpad
- And more!

All displayed with pretty colors and helpful information.

## 💻 My Learning Journey

I created this project to:
- Understand how Linux handles input devices
- Learn Rust programming (this is my first real Rust project!)
- Play with terminal colors and interfaces
- Share what I've learned with others

While working on this, I discovered how to:
- Use Rust's pattern matching
- Create a clean terminal UI
- Work with system input libraries
- Handle different types of events

## 📋 Features I Implemented

- **Colorful Display**: Different events show up in different colors!
- **Device Detection**: Shows when you plug in/remove keyboards, mice, etc.
- **Keyboard Tracking**: Shows exactly which key you pressed
- **Mouse Monitoring**: Tracks position, clicks, and scrolling
- **Touch & Gesture Support**: Works with touchpads and touchscreens
- **Stats Counter**: Keeps track of how many keys you've pressed and clicks you've made

## 🛠️ How to Install

This is a Rust project, so you'll need to have Rust installed. Then:

1. Clone this repo:
   ```
   git clone https://github.com/Nivesh12345/rustkey.git
   cd rustkey/libinput_project
   ```

2. Build it:
   ```
   cargo build
   ```

3. You'll need these system packages too:

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

## 🚀 Running It

Since this needs to access your input devices, you'll need to run it with sudo:

```
sudo ./target/debug/libinput_project
```

For better performance:
```
cargo build --release
sudo ./target/release/libinput_project
```

## 🌈 What You'll See

When you run it, you'll get a beautiful display like this:

```
══════════════════════════════════════════════════════
           🎮 RUSTKEY INPUT MONITOR 🎮           
══════════════════════════════════════════════════════

A beautiful way to visualize your input events in real-time

Waiting for input events... (press Ctrl+C to exit)

------------------------------------------
➕ Device Added
🖱️  Mouse absolute position: (1254.23, 876.49)
⌨️  KEY PRESS DETECTED --> A <-- (code: 30)
🔠 YOU PRESSED: [ A ] (Total key presses: 1)
🖱️  Mouse button LEFT (272) - PRESSED at position: (1254.23, 876.49) (Total clicks: 1)
```

Just press Ctrl+C when you want to quit.

## 🤔 Problems I Solved

While making this, I had to figure out:

1. How to get key names instead of just key codes
2. How to track absolute vs. relative mouse movements
3. How to add colors to make the output easier to understand
4. How to detect different types of devices

It was challenging but fun!

## 📝 What I Learned

This project taught me a ton about:
- Rust's ownership model
- Working with system libraries
- Terminal UI design
- How input devices communicate with our computers
- Using Git and GitHub for project management

## 🙏 Thanks

Special thanks to:
- The Rust community for their great documentation
- The creators of the libinput library
- Everyone who helped me learn Rust

<<<<<<< HEAD
## 🔍 Troubleshooting
=======
<<<<<<< HEAD
## 📄 License

This project is under the MIT License - see the LICENSE file for details.

---

If you have any questions or suggestions, please open an issue! I'm still learning and would love your feedback. 😊 
=======


## Troubleshooting
>>>>>>> c455b49d5e2589a34640d0bb7360588dc312b057

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

## 📄 License

This project is under the MIT License - see the LICENSE file for details.

## 👏 Acknowledgments

- The libinput team for their excellent input handling library
- The Rust community for the robust ecosystem 
<<<<<<< HEAD

---

If you have any questions or suggestions, please open an issue! I'm still learning and would love your feedback. 😊
=======
>>>>>>> 84f6dc9399bd34c63a7b76f0222e801746282bf4
>>>>>>> c455b49d5e2589a34640d0bb7360588dc312b057
