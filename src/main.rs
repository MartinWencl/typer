use std::io::{Write, stdout};
use crossterm::{ExecutableCommand, cursor};

fn main() {
    let mut stdout = stdout();
    stdout.execute(cursor::MoveTo(5,5));
}
