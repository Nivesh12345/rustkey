/*
 * RustKey Input Monitor
 * A lightweight, beautiful input event monitoring tool for Linux systems
 * 
 * Created by: Nivesh
 * GitHub: https://github.com/yourusername/rustkey
 * 
 * This tool visualizes keyboard and mouse events in real-time,
 * helping you understand how input devices communicate with your system.
 * It's perfect for debugging, learning, or just having fun watching your inputs!
 */

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

// Use constants directly instead of importing from libc
const O_RDONLY: i32 = 0;
const O_WRONLY: i32 = 1;
const O_RDWR: i32 = 2;

// Track current mouse position
struct MouseState {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
}

// Colors for terminal output (ANSI color codes)
struct Colors;
impl Colors {
    const RESET: &'static str = "\x1b[0m";
    const RED: &'static str = "\x1b[31m";
    const GREEN: &'static str = "\x1b[32m";
    const YELLOW: &'static str = "\x1b[33m";
    const BLUE: &'static str = "\x1b[34m";
    const MAGENTA: &'static str = "\x1b[35m";
    const CYAN: &'static str = "\x1b[36m";
    const BOLD: &'static str = "\x1b[1m";
}

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

// Helper function to convert key codes to more readable names
fn key_name(key_code: u32) -> &'static str {
    match key_code {
        1 => "ESC",
        28 => "ENTER",
        14 => "BACKSPACE",
        15 => "TAB",
        57 => "SPACE",
        // Letters
        16 => "Q", 17 => "W", 18 => "E", 19 => "R", 20 => "T", 
        21 => "Y", 22 => "U", 23 => "I", 24 => "O", 25 => "P",
        30 => "A", 31 => "S", 32 => "D", 33 => "F", 34 => "G", 
        35 => "H", 36 => "J", 37 => "K", 38 => "L",
        44 => "Z", 45 => "X", 46 => "C", 47 => "V", 48 => "B", 
        49 => "N", 50 => "M",
        // Arrow keys
        103 => "UP", 105 => "LEFT", 106 => "RIGHT", 108 => "DOWN",
        // Function keys
        59 => "F1", 60 => "F2", 61 => "F3", 62 => "F4",
        63 => "F5", 64 => "F6", 65 => "F7", 66 => "F8",
        67 => "F9", 68 => "F10", 87 => "F11", 88 => "F12",
        // Modifiers
        29 => "CTRL", 42 => "SHIFT (LEFT)", 54 => "SHIFT (RIGHT)",
        56 => "ALT", 100 => "ALT GR", 125 => "SUPER/WIN",
        // Other common keys
        2 => "1", 3 => "2", 4 => "3", 5 => "4", 6 => "5",
        7 => "6", 8 => "7", 9 => "8", 10 => "9", 11 => "0",
        12 => "-", 13 => "=", 26 => "[", 27 => "]", 39 => ";",
        40 => "'", 41 => "`", 43 => "\\", 51 => ",", 52 => ".",
        53 => "/", 58 => "CAPS LOCK", 69 => "NUM LOCK", 70 => "SCROLL LOCK",
        // Numpad
        71 => "NUM 7", 72 => "NUM 8", 73 => "NUM 9", 74 => "NUM -",
        75 => "NUM 4", 76 => "NUM 5", 77 => "NUM 6", 78 => "NUM +",
        79 => "NUM 1", 80 => "NUM 2", 81 => "NUM 3", 82 => "NUM 0",
        83 => "NUM .", 96 => "NUM ENTER", 98 => "NUM /", 55 => "NUM *",
        // Media keys
        113 => "MUTE", 114 => "VOLUME DOWN", 115 => "VOLUME UP",
        // System keys
        99 => "PRINT SCREEN", 119 => "PAUSE", 110 => "HOME", 102 => "PAGE UP",
        107 => "END", 109 => "PAGE DOWN", 111 => "DELETE", 118 => "INSERT",
        // Modern keys
        127 => "PAUSE", 128 => "PREV TRACK", 129 => "NEXT TRACK", 
        130 => "STOP", 131 => "PLAY/PAUSE",
        // Others
        _ => "UNKNOWN KEY",
    }
}

fn display_welcome_message() {
    // Clear screen
    print!("\x1c");
    
    println!("{}══════════════════════════════════════════════════════{}",
        Colors::CYAN, Colors::RESET);
    println!("{}{}           🎮 RUSTKEY INPUT MONITOR 🎮           {}{}",
        Colors::CYAN, Colors::BOLD, Colors::RESET, Colors::CYAN);
    println!("{}══════════════════════════════════════════════════════{}",
        Colors::CYAN, Colors::RESET);
    println!();
    println!("{}A beautiful way to visualize your input events in real-time{}", 
        Colors::GREEN, Colors::RESET);
    println!();
    println!("{}Waiting for input events...{} (press {}Ctrl+C{} to exit)", 
        Colors::YELLOW, Colors::RESET, Colors::RED, Colors::RESET);
    println!();
    println!("{}------------------------------------------{}", 
        Colors::CYAN, Colors::RESET);
    
    // Force flush to ensure everything is displayed
    io::stdout().flush().unwrap();
}

fn main() {
    // Initialize libinput
    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    
    // Show our fancy welcome message
    display_welcome_message();

    // Track mouse state
    let mut mouse_state = MouseState {
        x: 0.0,
        y: 0.0,
        dx: 0.0,
        dy: 0.0,
    };

    // Track basic statistics
    let mut key_press_count = 0;
    let mut mouse_click_count = 0;
    
    // Main event loop
    loop {
        input.dispatch().unwrap();
        
        for event in &mut input {
            match event {
                input::Event::Device(device_event) => {
                    match device_event {
                        input::event::DeviceEvent::Added(_device) => {
                            println!("{}➕ Device Added{}", Colors::GREEN, Colors::RESET);
                        },
                        input::event::DeviceEvent::Removed(_device) => {
                            println!("{}➖ Device Removed{}", Colors::RED, Colors::RESET);
                        },
                        _ => println!("{}📱 Other Device Event{}", Colors::BLUE, Colors::RESET),
                    }
                },
                input::Event::Keyboard(keyboard_event) => {
                    if let input::event::KeyboardEvent::Key(key_event) = keyboard_event {
                        // Get the key code directly from key_event method
                        let key_code = key_event.key();
                        let key_text = key_name(key_code);
                        
                        // Use key_state instead of state
                        if key_event.key_state() == input::event::keyboard::KeyState::Pressed {
                            key_press_count += 1;
                            println!("{}⌨️  KEY PRESS DETECTED --> {}{}{} {}{} {}<-- (code: {}){}",
                                Colors::YELLOW, 
                                Colors::MAGENTA, Colors::BOLD, 
                                key_text, 
                                Colors::RESET, Colors::YELLOW,
                                Colors::BOLD,
                                key_code,
                                Colors::RESET);
                            println!("{}🔠 YOU PRESSED: [ {} ]{} (Total key presses: {})",
                                Colors::GREEN, key_text, Colors::RESET, key_press_count);
                        } else {
                            println!("{}⌨️  KEY RELEASE DETECTED --> {} {} <-- (code: {}){}",
                                Colors::BLUE, key_text, Colors::RESET, key_code, Colors::RESET);
                        }
                    } else {
                        println!("{}⌨️  Other Keyboard Event{}", Colors::CYAN, Colors::RESET);
                    }
                },
                input::Event::Pointer(pointer_event) => {
                    match pointer_event {
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
                        input::event::PointerEvent::MotionAbsolute(abs_motion) => {
                            // Update absolute mouse position
                            mouse_state.x = abs_motion.absolute_x();
                            mouse_state.y = abs_motion.absolute_y();
                            
                            println!("{}🖱️  Mouse absolute position: ({:.2}, {:.2}){}",
                                Colors::CYAN, mouse_state.x, mouse_state.y, Colors::RESET);
                        },
                        input::event::PointerEvent::Button(button) => {
                            // Extract button number from debug representation
                            let debug_str = format!("{:?}", button);
                            let mut description = String::from("🖱️  Mouse button");
                            let mut button_num = 0;
                            
                            if debug_str.contains("button: ") {
                                if let Some(btn_start) = debug_str.find("button: ") {
                                    if let Some(btn_end) = debug_str[btn_start..].find(",") {
                                        let btn_str = &debug_str[btn_start+8..btn_start+btn_end];
                                        if let Ok(btn) = btn_str.parse::<u32>() {
                                            button_num = btn;
                                            // Map button numbers to common names
                                            let button_name = match btn {
                                                272 => "LEFT",
                                                273 => "RIGHT",
                                                274 => "MIDDLE",
                                                275 => "SIDE",
                                                276 => "EXTRA",
                                                _ => btn_str,
                                            };
                                            description = format!("🖱️  Mouse button {} ({})", button_name, btn);
                                        } else {
                                            description = format!("🖱️  Mouse button {}", btn_str);
                                        }
                                    }
                                }
                            }
                            
                            if debug_str.contains("Pressed") {
                                mouse_click_count += 1;
                                description.push_str(" - PRESSED");
                                println!("{}{} at position: ({:.2}, {:.2}){} (Total clicks: {})",
                                    Colors::MAGENTA, description, 
                                    mouse_state.x, mouse_state.y, 
                                    Colors::RESET, mouse_click_count);
                            } else if debug_str.contains("Released") {
                                description.push_str(" - RELEASED");
                                println!("{}{} at position: ({:.2}, {:.2}){}",
                                    Colors::BLUE, description, 
                                    mouse_state.x, mouse_state.y, 
                                    Colors::RESET);
                            }
                        },
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
                            
                            println!("{}🖱️  Scroll wheel: horizontal: {:.2}, vertical: {:.2}{}",
                                Colors::CYAN, scroll_x, scroll_y, Colors::RESET);
                        },
                        input::event::PointerEvent::ScrollFinger(_) => {
                            println!("{}🖱️  Scroll finger event{}", Colors::CYAN, Colors::RESET);
                        },
                        input::event::PointerEvent::ScrollContinuous(_) => {
                            println!("{}🖱️  Scroll continuous event{}", Colors::CYAN, Colors::RESET);
                        },
                        other => {
                            // Show the event type to help diagnose
                            let debug_str = format!("{:?}", other);
                            // Extract the variant name
                            if let Some(end) = debug_str.find('(') {
                                let variant = &debug_str[..end];
                                println!("{}🖱️  Pointer Event: {}{}", Colors::CYAN, variant, Colors::RESET);
                            } else {
                                println!("{}🖱️  Other Pointer Event: {:?}{}", Colors::CYAN, other, Colors::RESET);
                            }
                        },
                    }
                },
                input::Event::Touch(touch_event) => {
                    let debug_str = format!("{:?}", touch_event);
                    if let Some(end) = debug_str.find('(') {
                        let variant = &debug_str[..end];
                        println!("{}👆 Touch Event: {}{}", Colors::MAGENTA, variant, Colors::RESET);
                    } else {
                        println!("{}👆 Touch Event: {:?}{}", Colors::MAGENTA, touch_event, Colors::RESET);
                    }
                },
                input::Event::Gesture(gesture_event) => {
                    let debug_str = format!("{:?}", gesture_event);
                    if let Some(end) = debug_str.find('(') {
                        let variant = &debug_str[..end];
                        println!("{}🤲 Gesture Event: {}{}", Colors::MAGENTA, variant, Colors::RESET);
                    } else {
                        println!("{}🤲 Gesture Event: {:?}{}", Colors::MAGENTA, gesture_event, Colors::RESET);
                    }
                },
                input::Event::Tablet(_) => println!("{}✏️ Tablet Event{}", Colors::YELLOW, Colors::RESET),
                input::Event::Switch(_) => println!("{}🔄 Switch Event{}", Colors::YELLOW, Colors::RESET),
                _ => println!("{}⚠️ Other Event{}", Colors::RED, Colors::RESET),
            }
        }
        
        sleep(Duration::from_millis(5)); // Small sleep to reduce CPU usage
    }
}
