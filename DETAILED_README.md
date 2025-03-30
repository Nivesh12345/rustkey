# 🔍 RustKey Input Monitor - My Coding Journey

Hey there, fellow coder! 👋 This is my little guide to help you understand how I built this input monitoring tool. I'll walk through my code line by line and explain everything in a way that hopefully makes sense, even if you're new to Rust like me!

## 🤔 What Was I Trying to Create?

I wanted to make something that could show me what's happening when I press keys or move my mouse. Think of it like a magical window into how your computer "sees" the things you do with your keyboard and mouse. I was curious about this, and thought it might help other people learn too!

## 🧩 Let's Break Down My Code

### 📚 The Import Section

```rust
use input::{Libinput, LibinputInterface};
use input::event::pointer::Axis;
use input::event::keyboard::KeyboardEventTrait;
use input::event::pointer::PointerScrollEvent;
use std::fs::OpenOptions;
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;
use std::time::Duration;
use std::thread::sleep;
use std::io::{self, Write};
```

This part is like making a shopping list before cooking! Before I could start coding, I needed to tell Rust which special tools or "libraries" I wanted to use:

- The `input` stuff helps me talk to keyboards and mice
- `Path` helps find files on the computer
- `Duration` and `sleep` let me make my program pause briefly
- `io::Write` helps me display things nicely in the terminal

### 🔢 Constants for File Access

```rust
// Use constants directly instead of importing from libc
const O_RDONLY: i32 = 0;
const O_WRONLY: i32 = 1;
const O_RDWR: i32 = 2;
```

These are like secret codes I use when I need to talk to devices. It's easier to remember "O_RDONLY" than just "0", right? 
- `O_RDONLY` means "I only want to read information" (like listening)
- `O_WRONLY` means "I only want to send information" (like talking)
- `O_RDWR` means "I want to both send and receive" (like having a conversation)

### 📝 Tracking Mouse Position

```rust
// Track current mouse position
struct MouseState {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
}
```

I created a special notebook to keep track of where the mouse is:
- `x` and `y` tell me the current position (like coordinates on a map)
- `dx` and `dy` tell me how much it just moved (like "2 steps east, 3 steps north")

The `f64` just means these numbers can have decimal points (like 123.45).

### 🎨 Colors for Terminal

```rust
// Colors for terminal output (ANSI color codes)
struct Colors;
impl Colors {
    const RESET: &'static str = "\x1b[0m";
    const RED: &'static str = "\x1b[31m";
    const GREEN: &'static str = "\x1b[32m";
    // ... more colors ...
}
```

This is my paint palette! I define different colors I can use to make the output pretty. These weird codes like `\x1b[31m` are special instructions that tell the terminal "Hey, start printing in red now!" 

I spent a lot of time picking which colors to use for different things, and I'm pretty happy with how it turned out!

### 🔌 The Interface to Talk with Devices

```rust
struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        // ... code to open a device ...
    }
    fn close_restricted(&mut self, _fd: OwnedFd) {
        // OwnedFd automatically closes when dropped
    }
}
```

This was a tricky part! It's like teaching my program how to say "hello" and "goodbye" to the keyboard and mouse. The `open_restricted` function is how I start listening to a device, and `close_restricted` is how I politely end the conversation when I'm done.

### 📋 The Dictionary of Key Names

```rust
// Helper function to convert key codes to more readable names
fn key_name(key_code: u32) -> &'static str {
    match key_code {
        1 => "ESC",
        28 => "ENTER",
        // ... many more keys ...
        _ => "UNKNOWN KEY",
    }
}
```

This function is like a translator! When you press a key, the computer doesn't actually see "A" or "ENTER" - it sees numbers like 30 or 28. I made this function to convert those boring numbers into friendly names we can understand. 

The `match` keyword is super cool - it's like saying "if the key_code is 1, return 'ESC', if it's 28, return 'ENTER', and so on..."

### 💫 Welcome Message Function

```rust
fn display_welcome_message() {
    // Clear screen
    print!("\x1c");
    
    println!("{}══════════════════════════════════════════════════════{}",
        Colors::CYAN, Colors::RESET);
    // ... more pretty welcome text ...
    
    // Force flush to ensure everything is displayed
    io::stdout().flush().unwrap();
}
```

This was my attempt to make a good first impression! The function displays a fancy welcome message with colors and borders. I even used emojis to make it more fun! 

The `flush()` part makes sure everything gets displayed right away. Without it, sometimes the welcome message might not show up until later events happen, which would be weird.

### 🚀 The Main Function - Where Everything Starts

```rust
fn main() {
    // Initialize libinput
    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    
    // Show our fancy welcome message
    display_welcome_message();

    // Track mouse state
    let mut mouse_state = MouseState {
        x: 0.0, y: 0.0, dx: 0.0, dy: 0.0,
    };

    // Track basic statistics
    let mut key_press_count = 0;
    let mut mouse_click_count = 0;
    
    // ... the main loop comes next ...
}
```

This is where my program actually starts running! I:
1. Set up the input library to listen to devices
2. Display the welcome message
3. Create the mouse position tracker starting at (0,0)
4. Set up counters to keep track of key presses and mouse clicks

The `mut` keyword means these variables can change over time, which makes sense because the mouse position and counts will definitely change!

### 🔄 The Never-Ending Loop

```rust
// Main event loop
loop {
    input.dispatch().unwrap();
    
    for event in &mut input {
        // ... handle different events ...
    }
    
    sleep(Duration::from_millis(5)); // Small sleep to reduce CPU usage
}
```

This is like the heartbeat of my program! The `loop` keyword means "keep doing this forever" (until someone presses Ctrl+C to exit). Every cycle:

1. I check for new events with `input.dispatch()`
2. I look at each event that happened
3. I take a tiny nap (5 milliseconds) to save power

Without that sleep, my program would use 100% of a CPU core, which seemed wasteful!

### 📱 Handling Device Events

```rust
match event {
    input::Event::Device(device_event) => {
        match device_event {
            input::event::DeviceEvent::Added(_device) => {
                println!("{}➕ Device Added{}", Colors::GREEN, Colors::RESET);
            },
            // ... handle other device events ...
        }
    },
    // ... handle other event types ...
}
```

This is how I detect when devices are connected or disconnected. The `match` statement is like a smarter version of "if/else if/else" - it helps me handle different types of events in different ways.

I use green text for device connections because it seemed like a positive thing!

### ⌨️ Handling Keyboard Events

```rust
input::Event::Keyboard(keyboard_event) => {
    if let input::event::KeyboardEvent::Key(key_event) = keyboard_event {
        // Get the key code directly from key_event method
        let key_code = key_event.key();
        let key_text = key_name(key_code);
        
        // Use key_state instead of state
        if key_event.key_state() == input::event::keyboard::KeyState::Pressed {
            key_press_count += 1;
            println!("{}⌨️  KEY PRESS DETECTED --> {}{}{} {}{} {}<-- (code: {}){}",
                // ... lots of color formatting ...
            );
            println!("{}🔠 YOU PRESSED: [ {} ]{} (Total key presses: {})",
                Colors::GREEN, key_text, Colors::RESET, key_press_count);
        } else {
            // ... handle key release ...
        }
    } else {
        // ... handle other keyboard events ...
    }
},
```

This was one of my favorite parts to write! When a key is pressed:
1. I get the key code and translate it to a name
2. I check if the key was pressed down or released
3. I print a fancy message with colors and emojis
4. I update the counter if it was pressed

I spent extra time making this part visually distinct so it's easy to see which key was pressed.

### 🖱️ Handling Mouse Movements

```rust
input::event::PointerEvent::Motion(motion) => {
    // Update mouse state
    mouse_state.dx = motion.dx();
    mouse_state.dy = motion.dy();
    mouse_state.x += motion.dx();
    mouse_state.y += motion.dy();
    
    println!("{}🖱️  Mouse motion - Position: ({:.2}, {:.2}), Delta: ({:.2}, {:.2}){}",
        Colors::CYAN, 
        mouse_state.x, mouse_state.y, 
        mouse_state.dx, mouse_state.dy, 
        Colors::RESET);
},
```

This part updates the mouse position when it moves. The `.dx()` and `.dy()` functions tell me how much the mouse moved horizontally and vertically. I add these values to the current position to keep track of where the mouse is.

The `{:.2}` in the `println!` means "show these numbers with 2 decimal places" so they look nicer.

### 👆 Handling Mouse Buttons

```rust
input::event::PointerEvent::Button(button) => {
    // Extract button number and convert to name...
    
    if debug_str.contains("Pressed") {
        mouse_click_count += 1;
        description.push_str(" - PRESSED");
        println!("{}{} at position: ({:.2}, {:.2}){} (Total clicks: {})",
            Colors::MAGENTA, description, 
            mouse_state.x, mouse_state.y, 
            Colors::RESET, mouse_click_count);
    } else if debug_str.contains("Released") {
        // ... handle button release ...
    }
},
```

This detects mouse clicks! I had to do some string parsing to figure out which button was pressed (LEFT, RIGHT, MIDDLE, etc.). When a button is pressed:
1. I increment the click counter
2. I format a description with the button name and current mouse position
3. I print it in a nice color

The hardest part was figuring out the button numbers - 272 is left click, 273 is right click, etc.

### 🛑 Taking a Tiny Nap

```rust
sleep(Duration::from_millis(5)); // Small sleep to reduce CPU usage
```

This is such a simple line, but it makes a big difference! Without it, my program would use 100% CPU. With it, it uses almost nothing. The program still feels instant because 5 milliseconds is so short.

## 🧠 How It All Works Together

1. I set up the system to monitor input devices
2. I start an infinite loop to keep checking for events
3. When events happen, I sort them by type (keyboard, mouse, etc.)
4. I format the information in a pretty way with colors and emojis
5. I print it to the terminal so you can see it
6. I keep track of statistics like how many keys you've pressed

## 🎁 Extra Features I'm Proud Of

1. **Colors!** - Each type of event has its own color, making it easy to scan visually
2. **Counters** - I keep track of how many keys you've pressed and how many times you've clicked
3. **Detailed Position Info** - You can see exactly where your mouse is and how it moved
4. **Pretty Formatting** - The borders and emojis make it much nicer to look at
5. **Performance** - The tiny sleep makes it use very little CPU while still being responsive

## 💭 What I Want to Add Next

- Logging events to a file
- Configuration options (like changing colors)
- A graphical interface instead of just text
- Support for more types of devices
- Heatmaps showing which keys you use most often

## 🎓 What I Learned Making This

I was completely new to Rust when I started this project, so I learned tons!
- How Rust's ownership system works
- How to use external libraries
- How to work with system-level APIs
- How to format terminal output with colors
- How to handle different types of events

## 🤗 Final Thoughts

This project was super fun to build! I'm still learning Rust, so there are probably better ways to do some of these things. If you have suggestions, I'd love to hear them!

I hope this explanation helps you understand my code, and maybe even inspires you to try making something similar. Happy coding!

~ Nivesh 