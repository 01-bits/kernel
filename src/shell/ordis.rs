use crate::println;
use crate::print_info;
use crate::sync::Spinlock;

pub static SHELL: Spinlock<KShell> = Spinlock::new(KShell::new());

pub struct KShell {
    buffer: [u8; 64],
    cursor: usize,
}

impl KShell {
    pub const fn new() -> Self {
        Self {
            buffer: [0; 64],
            cursor: 0,
        }
    }

    pub fn handle_char(&mut self, c: char) {
        match c {
            '\n' => {
                println!(""); 
                self.execute(); // This is your 'interpret'
                self.cursor = 0; 
                print_info!("> "); 
            }
            // Handling Backspace (AZERTY scancode 0x0E)
            '\x08' => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    // We need to actually erase the char on screen
                    crate::drivers::vga::backspace(); 
                }
            }
            _ => {
                if self.cursor < 64 {
                    self.buffer[self.cursor] = c as u8;
                    self.cursor += 1;
                    print_info!("{}", c); 
                }
            }
        }
    }

    fn execute(&self) {
        // Only look at the bytes we actually typed
        let cmd = &self.buffer[..self.cursor];

        match cmd {
            b"help" => {
                println!("KShell v0.1 - Commands: help, info, clear");
            }
            b"clear" => {
                crate::drivers::vga::clear_screen();
            }
            b"info" => {
                println!("Arch: x86_64 | Layout: AZERTY | Logic: Consolidated");
            }
            b"" => {}, // Do nothing on empty enter
            _ => {
                println!("Error: Command not found.");
            }
        }
    }
}