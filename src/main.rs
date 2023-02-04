use std::io::{self, stdin, Cursor, Write};

use crate::lexer::tokenize;

mod lexer;

fn main() {
    let buf = String::new();
    let mut cursor = Cursor::new(buf);
    loop {
        cursor.get_mut().clear();
        print!("rlisp> ");
        let _ = io::stdout().flush();
        match stdin().read_line(cursor.get_mut()) {
            Ok(i) => println!("output: {:?}", tokenize(cursor.get_ref().as_str())),
            Err(e) => println!("Error: {}", e),
        }
    }
}
