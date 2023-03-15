mod repl;
use repl::Terminal;

fn main() {
    let mut terminal = Terminal::new();
    terminal.run();
}
