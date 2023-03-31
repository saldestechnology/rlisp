use libc::{c_int, tcgetattr, tcsetattr, termios, ECHO, ICANON, TCSANOW};
use std::io::{Read, Write};

trait IntoRaw {
    fn enable_raw_mode(&mut self) -> ();
    fn disable_raw_mode(&mut self) -> ();
}

/**
 * Moving enable_raw_mode and disable_raw_mode to a trait
 * I've heard it's possible to accomplish raw mode without libc
 * but I'm not sure how to do that.
 * But this way it should be easy enough to replace libc with something else.
 */
impl<R: Read, W: Write> IntoRaw for Terminal<R, W> {
    fn enable_raw_mode(&mut self) {
        let mut new_termios = self.original_termios;
        new_termios.c_lflag &= !(ECHO | ICANON);

        unsafe {
            tcsetattr(self.stdin_fd, TCSANOW, &new_termios as *const _);
        }
    }

    fn disable_raw_mode(&mut self) {
        unsafe {
            tcsetattr(self.stdin_fd, TCSANOW, &self.original_termios as *const _);
        }
    }
}

pub struct Terminal<R: Read, W: Write> {
    stdin: R,
    stdout: W,
    stdin_fd: c_int,
    original_termios: termios,
    buffer: String,
    cursor_pos: usize,
}

impl<R: Read, W: Write> Terminal<R, W> {
    pub fn new(stdin: R, stdout: W, stdin_fd: c_int, original_termios: termios) -> Terminal<R, W> {
        Terminal {
            stdin,
            stdout,
            stdin_fd,
            original_termios,
            buffer: String::new(),
            cursor_pos: 0,
        }
    }

    fn handle_key_event(&mut self, key: u8) {
        match key {
            b'\x1b' => {
                let mut buf = [0u8; 2];
                let _ = self.stdin.read(&mut buf);
                if buf[0] == b'[' {
                    match buf[1] {
                        b'C' => self.move_cursor_right(),
                        b'D' => self.move_cursor_left(),
                        _ => (),
                    }
                }
            }
            0x7f => self.backspace(),
            _ => self.insert_char(key as char),
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_pos < self.buffer.len() {
            self.cursor_pos += 1;
            self.stdout.write_all(b"\x1b[1C").unwrap();
            self.stdout.flush().unwrap();
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.stdout.write_all(b"\x1b[1D").unwrap();
            self.stdout.flush().unwrap();
        }
    }

    fn insert_char(&mut self, c: char) {
        self.buffer.insert(self.cursor_pos, c);
        self.stdout.write_all(b"\x1b[s").unwrap(); // Save cursor position
        self.stdout
            .write_all(&self.buffer[self.cursor_pos..].as_bytes())
            .unwrap(); // Write the updated buffer from cursor position
        self.cursor_pos += 1;
        self.stdout.write_all(b"\x1b[u").unwrap(); // Restore cursor position
        self.stdout.write_all(b"\x1b[1C").unwrap(); // Move cursor one position to the right
        self.stdout.flush().unwrap();
    }

    fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.buffer.remove(self.cursor_pos - 1);
            self.cursor_pos -= 1;
            self.stdout.write_all(b"\x1b[1D").unwrap(); // Move cursor one position to the left
            self.stdout.write_all(b"\x1b[s").unwrap(); // Save cursor position
            self.stdout.write_all(b"\x1b[K").unwrap(); // Clear from cursor to end of line
            self.stdout
                .write_all(&self.buffer[self.cursor_pos..].as_bytes())
                .unwrap(); // Write the updated buffer from cursor position
            self.stdout.write_all(b"\x1b[u").unwrap(); // Restore cursor position
            self.stdout.flush().unwrap();
        }
    }

    pub fn run(&mut self) {
        IntoRaw::enable_raw_mode(self);
        loop {
            let mut buf = [0u8; 1];
            let read_bytes = self.stdin.read(&mut buf).unwrap();
            if buf[0] == b'q' {
                break;
            } else {
                self.handle_key_event(buf[0]);
            }
        }
        IntoRaw::disable_raw_mode(self);
    }
}
