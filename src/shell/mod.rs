use crate::{VGA_HEIGHT, VGA_WIDTH, print_info};

pub struct CommandBuffer {
    buf: [u8; 64],
    pos: usize,
}

impl CommandBuffer {
    pub const fn new() -> Self {
        Self {
            buf: [0; 64],
            pos: 0,
        }
    }

    pub fn add_char(&mut self, c: u8) {
        if c == b'\n' {
            self.interpret();
            self.pos = 0;
        } else if self.pos < 64 {
            self.buf[self.pos] = c;
            self.pos += 1;
        }
    }

    fn interpret(&self) {
        if self.buf.starts_with(b"help") {
            crate::println!("\nAvailable commands: help, clear, info");
        } else if self.buf.starts_with(b"clear") {
            clear();
        } else {
            crate::println!("\nUnknown command.");
        }
    }
}

fn clear() {
    for _ in 0..VGA_HEIGHT {
        for a in 0..VGA_WIDTH {
            print_info!(" ")
        }
    }
}
