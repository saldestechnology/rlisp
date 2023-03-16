mod repl;
use repl::Terminal;

use libc::{c_int, tcgetattr, tcsetattr, termios, ECHO, ICANON, TCSANOW};
use std::io::{self, stdin, stdout};
use std::os::unix::io::AsRawFd;

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let stdin_fd = stdin.as_raw_fd();
    let original_termios = unsafe {
        let mut termios: termios = std::mem::zeroed();
        tcgetattr(stdin_fd, &mut termios as *mut _);
        termios
    };

    let mut terminal = Terminal::new(stdin.lock(), stdout.lock(), stdin_fd, original_termios);
    terminal.run();
}
