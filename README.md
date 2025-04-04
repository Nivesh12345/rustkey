# 🎮 RustKey Input Monitor

Hey there!. I built this input monitoring tool that shows you all your keyboard and mouse actions in real-time with pretty colors. This was both a learning exercise for me and something I thought might be useful for others.

## ✨ What Does It Do?

My program watches your keyboard and mouse inputs and displays them in a colorful way in your terminal. Here's what it can show:

- 🖮 **Keyboard Stuff**: See exactly which keys you're pressing with fun colorful messages
- 🖱️ **Mouse Movements**: Track where your mouse is on screen and how it's moving
- 👆 **Clicks & Scrolls**: See when and where you click and scroll
- 📱 **Device Changes**: Know when devices connect or disconnect
- 👐 **Touch & Gestures**: Works with touchscreens and touchpads too!

I was really proud when I got the colors working - it makes everything so much easier to understand!

## 📋 What You'll Need

To run my project, you'll need:

- A Linux computer (I'm using Arch Linux but other distros should work too)
- Rust and Cargo installed
- The libinput development package
- Admin (sudo) rights since we need to access input devices

### 📦 Installing Dependencies

I had to install some packages first. Depending on your Linux flavor, run one of these:

For Arch Linux (that's what I use!):
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

## 🚀 Getting Started

I was really excited when I finally got this working! Here's how you can try it too:

1. First, grab my code:
   ```
   git clone https://github.com/Nivesh12345/rustkey.git
   cd rustkey/libinput_project
   ```

2. Build the project (this might take a minute):
   ```
   cargo build
   ```

3. Run it (needs sudo to access your input devices):
   ```
   sudo ./target/debug/libinput_project
   ```

If you want a faster version for everyday use, you can build an optimized release:
```
cargo build --release
sudo ./target/release/libinput_project
```

## 🎨 What You'll See

When you run it, you'll get this pretty display in your terminal:

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

I spent a lot of time making the colors look nice! Green for good things, red for warnings, cyan for mouse stuff, and so on. The program also counts how many keys you've pressed and how many times you've clicked, which I thought was a cool addition.

## 🛠️ Things I Learned

While making this project, I learned a ton about:

1. How Rust works with system inputs
2. Adding colors to terminal output
3. Working with real-time events
4. Converting raw input codes to human-readable text
5. Tracking statistics of user behavior

<!--I'm still new to Rust, so there might be better ways to do some things - feel free to suggest improvements!-->

## ❓ Troubleshooting

Some problems I ran into that you might face too:

- **"Permission denied" errors**: Make sure you're using sudo!
- **Input devices not showing up**: Try unplugging and plugging them back in
- **Colors look weird**: Some terminals don't support all the colors - try a different terminal app

## 📝 My Journey & Future Plans

This started as a simple experiment to learn Rust, but I got really into it! I want to add more features in the future like:

- Saving logs to a file
- Creating heatmaps of keyboard and mouse usage
- Adding configuration options
- Maybe making a GUI version someday

## 📄 License

This project is under the MIT License - see the LICENSE file for details.

---

Happy coding! 😊
~ Nivesh
