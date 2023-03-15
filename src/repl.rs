use libc::{c_int, tcgetattr, tcsetattr, termios, ECHO, ICANON, TCSANOW};
use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;

pub struct Terminal {
    stdin: io::Stdin,
    stdout: io::Stdout,
    original_termios: termios,
    buffer: String,
    cursor_pos: usize,
}

impl Terminal {
    pub fn new() -> Terminal {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let stdin_fd = stdin.as_raw_fd();

        let original_termios = unsafe {
            let mut termios: termios = std::mem::zeroed();
            tcgetattr(stdin_fd, &mut termios as *mut _);
            termios
        };

        Terminal {
            stdin,
            stdout,
            original_termios,
            buffer: String::new(),
            cursor_pos: 0,
        }
    }

    pub fn enable_raw_mode(&mut self) {
        let stdin_fd = self.stdin.as_raw_fd();
        let mut raw_termios = self.original_termios.clone();
        raw_termios.c_lflag &= !(ECHO as libc::tcflag_t | ICANON as libc::tcflag_t);
        unsafe { tcsetattr(stdin_fd, TCSANOW, &raw_termios as *const _) };
    }

    pub fn disable_raw_mode(&mut self) {
        let stdin_fd = self.stdin.as_raw_fd();
        unsafe { tcsetattr(stdin_fd, TCSANOW, &self.original_termios as *const _) };
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
        self.enable_raw_mode();
        loop {
            let mut buf = [0u8; 1];
            let read_bytes = self.stdin.read(&mut buf).unwrap();
            if buf[0] == b'q' {
                break;
            } else {
                self.handle_key_event(buf[0]);
            }
        }
        self.disable_raw_mode();
    }
}
