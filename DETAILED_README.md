# RustKey Input Monitor - Code Explanation for Beginners

Hello! This document will explain how our input monitoring program works, line by line, in simple terms. Think of this program as a curious little helper that watches what keys you press and how you move your mouse, and then tells you about it.

## What Does Our Program Do?

Our program is like a watchful friend who sits next to your keyboard and mouse and says things like "Oh! You just pressed the A key!" or "I see you're moving your mouse to the right!" It observes all your device inputs and reports them in a friendly way.

## Let's Break Down the Code

### The Import Section

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
```

Imagine you're building something with LEGO. Before you start, you need to gather all the pieces you'll need. In programming, we do this with "use" statements. These lines tell our program to grab special tools we need:

- `input::{Libinput, LibinputInterface}` - This is our main tool for detecting keypresses and mouse movements
- `input::event::pointer::Axis` - This helps us understand if the mouse is moving up/down or left/right
- `KeyboardEventTrait` - This gives us special powers to understand keyboard events
- `PointerScrollEvent` - This helps us understand when you scroll with your mouse wheel
- `OpenOptions`, `OpenOptionsExt`, and `OwnedFd` - These help us talk to your computer's devices
- `Path` - This helps us find files on your computer
- `Duration` and `sleep` - These help our program take short naps so it doesn't use too much power

### Constants for File Access

```rust
// Use constants directly instead of importing from libc
const O_RDONLY: i32 = 0;
const O_WRONLY: i32 = 1;
const O_RDWR: i32 = 2;
```

These are like special codes that tell the computer how we want to access devices:
- `O_RDONLY = 0` means "I only want to read from this device"
- `O_WRONLY = 1` means "I only want to write to this device"
- `O_RDWR = 2` means "I want to both read from and write to this device"

### Mouse State Tracking

```rust
// Track current mouse position
struct MouseState {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
}
```

Think of this as a special notebook where we write down:
- `x` - How far left or right the mouse is (horizontally)
- `y` - How far up or down the mouse is (vertically)
- `dx` - How much the mouse just moved left/right
- `dy` - How much the mouse just moved up/down

We use these to keep track of where your mouse is on the screen!

### Our Interface

```rust
struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, _fd: OwnedFd) {
        // OwnedFd automatically closes when dropped
    }
}
```

This part is like teaching our program how to talk to your keyboard and mouse. It's similar to how you need to know a special handshake to join a secret club:

- `open_restricted` is like our way of saying "Hello, keyboard! I'd like to know when keys are pressed!"
- It uses those special codes we defined earlier to determine if we're reading, writing, or both
- `close_restricted` is our way of saying "Goodbye, keyboard! Thanks for telling me about the keys!"

### Key Name Function

```rust
// Helper function to convert key codes to more readable names
fn key_name(key_code: u32) -> &'static str {
    match key_code {
        1 => "ESC",
        28 => "ENTER",
        14 => "BACKSPACE",
        // ... many more keys ...
        _ => "UNKNOWN KEY",
    }
}
```

This is like a translator! Your keyboard doesn't actually send letters like "A" or "B" - it sends numbers like 30 or 48. This function is like a dictionary that helps us translate:
- If we receive `1`, it means the ESC key was pressed
- If we receive `28`, it means the ENTER key was pressed
- And so on for many other keys!
- If we get a number we don't recognize, we call it "UNKNOWN KEY"

### Main Function

```rust
fn main() {
    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    
    println!("=== 📥 Input Event Monitor ===");
    println!("Waiting for input events... (press Ctrl+C to exit)");
    println!("------------------------------------------");

    // Track mouse state
    let mut mouse_state = MouseState {
        x: 0.0,
        y: 0.0,
        dx: 0.0,
        dy: 0.0,
    };
```

This is where our program starts working:

1. We create a special helper called `input` that can detect keyboard and mouse actions
2. We tell this helper to watch your main keyboard and mouse (that's what "seat0" means)
3. We print a nice welcome message with emojis
4. We create that special notebook to track mouse movements, starting at position (0,0)

### The Main Loop

```rust
loop {
    input.dispatch().unwrap();
    
    for event in &mut input {
        // ... handle different types of events ...
    }
    
    sleep(Duration::from_millis(5)); // Small sleep to reduce CPU usage
}
```

This is like saying "forever and ever, do the following":

1. `input.dispatch()` - Check if there are any new events (keypresses or mouse movements)
2. For each event we found, figure out what kind it is and report it
3. Take a tiny nap (5 milliseconds - that's 0.005 seconds!) to save power
4. Then start all over again!

### Handling Device Events

```rust
match event {
    input::Event::Device(device_event) => {
        match device_event {
            input::event::DeviceEvent::Added(_device) => {
                // DeviceAddedEvent doesn't have a name method we can use
                println!("➕ Device Added");
            },
            input::event::DeviceEvent::Removed(_device) => {
                // DeviceRemovedEvent doesn't have a name method we can use
                println!("➖ Device Removed");
            },
            _ => println!("📱 Other Device Event"),
        }
    },
```

This is where we handle when devices get connected or disconnected:

- If a new keyboard or mouse is plugged in, we print "➕ Device Added"
- If a keyboard or mouse is unplugged, we print "➖ Device Removed"
- If something else happens with a device, we print "📱 Other Device Event"

### Handling Keyboard Events

```rust
input::Event::Keyboard(keyboard_event) => {
    if let input::event::KeyboardEvent::Key(key_event) = keyboard_event {
        // Get the key code directly from key_event method
        let key_code = key_event.key();
        let key_text = key_name(key_code);
        
        // Use key_state instead of state
        if key_event.key_state() == input::event::keyboard::KeyState::Pressed {
            println!("⌨️  KEY PRESS DETECTED --> [ {} ] <-- (code: {})", key_text, key_code);
            println!("🔠 YOU PRESSED: [ {} ]", key_text);
        } else {
            println!("⌨️  KEY RELEASE DETECTED --> [ {} ] <-- (code: {})", key_text, key_code);
        }
    } else {
        println!("⌨️  Other Keyboard Event");
    }
},
```

This is the exciting part where we detect keypresses!

1. When a key event happens, we get the key code (a number)
2. We translate that number to a name using our translator function
3. We check if the key was pressed down or released
4. If pressed:
   - We print a fancy message like "⌨️ KEY PRESS DETECTED --> [ A ] <-- (code: 30)"
   - We also print "🔠 YOU PRESSED: [ A ]" to make it super clear
5. If released:
   - We print something like "⌨️ KEY RELEASE DETECTED --> [ A ] <-- (code: 30)"

### Handling Mouse Movement

```rust
input::event::PointerEvent::Motion(motion) => {
    // Update mouse state
    mouse_state.dx = motion.dx();
    mouse_state.dy = motion.dy();
    mouse_state.x += motion.dx();
    mouse_state.y += motion.dy();
    
    println!("🖱️  Mouse motion - Position: ({:.2}, {:.2}), Delta: ({:.2}, {:.2})", 
            mouse_state.x, mouse_state.y, mouse_state.dx, mouse_state.dy);
},
input::event::PointerEvent::MotionAbsolute(abs_motion) => {
    // Update absolute mouse position
    mouse_state.x = abs_motion.absolute_x();
    mouse_state.y = abs_motion.absolute_y();
    
    println!("🖱️  Mouse absolute position: ({:.2}, {:.2})", 
             mouse_state.x, mouse_state.y);
},
```

This part handles when you move your mouse:

1. `PointerEvent::Motion` is when you move your mouse relatively (like "I moved a little to the right")
   - We update our notebook with how much the mouse moved
   - We add this movement to the current position
   - We print the current position and how much it just moved

2. `PointerEvent::MotionAbsolute` is when the computer tells us exactly where the mouse is
   - We directly set our notebook to these exact coordinates
   - We print the absolute position of the mouse

### Handling Mouse Buttons

```rust
input::event::PointerEvent::Button(button) => {
    // Extract button number from debug representation
    let debug_str = format!("{:?}", button);
    let mut description = String::from("🖱️  Mouse button");
    let mut button_num = 0;
    
    if debug_str.contains("button: ") {
        // ... figure out which button was pressed ...
    }
    
    if debug_str.contains("Pressed") {
        description.push_str(" - PRESSED");
    } else if debug_str.contains("Released") {
        description.push_str(" - RELEASED");
    }
    
    // Add current position information
    description.push_str(&format!(" at position: ({:.2}, {:.2})", 
                        mouse_state.x, mouse_state.y));
    
    println!("{}", description);
},
```

This part detects when you click your mouse buttons:

1. We look at the button event information
2. We figure out which button it was (LEFT, RIGHT, MIDDLE, etc.)
3. We check if it was pressed or released
4. We add the current mouse position from our notebook
5. We print a message like "🖱️ Mouse button LEFT (272) - PRESSED at position: (123.45, 678.90)"

### Handling Mouse Wheel Scrolling

```rust
input::event::PointerEvent::ScrollWheel(scroll) => {
    // Get scroll information using the correct Axis enum
    // Fix the error by checking if axis is set first
    let scroll_y = if scroll.has_axis(Axis::Vertical) {
        scroll.scroll_value_v120(Axis::Vertical)
    } else {
        0.0
    };
    
    let scroll_x = if scroll.has_axis(Axis::Horizontal) {
        scroll.scroll_value_v120(Axis::Horizontal)
    } else {
        0.0
    };
    
    println!("🖱️  Scroll wheel: horizontal: {:.2}, vertical: {:.2}", 
             scroll_x, scroll_y);
},
```

This part handles when you use the scroll wheel on your mouse:

1. We check if the scroll event has vertical movement (up/down)
   - If yes, we get the amount scrolled
   - If no, we use 0.0 (no movement)
2. We do the same for horizontal scrolling (left/right, if your mouse supports it)
3. We print how much you scrolled in both directions

### Handling Touch and Other Events

```rust
input::Event::Touch(touch_event) => {
    // ... handle touch events ...
},
input::Event::Gesture(gesture_event) => {
    // ... handle gesture events ...
},
input::Event::Tablet(_) => println!("✏️ Tablet Event"),
input::Event::Switch(_) => println!("🔄 Switch Event"),
_ => println!("⚠️ Other Event"),
```

This part handles all other kinds of input:

1. Touch events - for touchscreens
2. Gesture events - for multi-finger gestures on trackpads
3. Tablet events - for drawing tablets
4. Switch events - for special switches
5. Any other events we didn't specifically program for

### The Small Sleep

```rust
sleep(Duration::from_millis(5)); // Small sleep to reduce CPU usage
```

This is our tiny nap at the end of each loop. Instead of constantly checking for new events as fast as possible (which would use a lot of power), we rest for 5 milliseconds. This makes our program more efficient without noticeably affecting how fast it responds to your actions.

## How It All Works Together

1. We set up our tools and notebook
2. We enter an endless loop
3. During each loop:
   - We check for new events
   - If we find any, we figure out what type they are
   - We handle each event appropriately (keyboard, mouse, etc.)
   - We print friendly messages about what happened
   - We take a tiny nap
   - Then we start checking again

This creates a program that constantly watches your input devices and tells you what's happening with them in a friendly, easy-to-understand way.

## Why Is This Useful?

This program can help you:
1. Learn how computers detect keypresses and mouse movements
2. Test if your keyboard and mouse are working correctly
3. See exactly what information your computer receives when you use input devices
4. Understand the relationship between key codes and the actual keys you press

It's like having X-ray vision to see what's happening inside your computer when you use it!

## How to Experiment With the Code

Try changing some things to see what happens:
1. Change the key names in the `key_name` function
2. Change the emoji icons to different ones
3. Modify the messages that get printed
4. Change the sleep time to see how it affects performance

Programming is all about experimenting and learning, so have fun with it! 