#[macro_use]
extern crate crossterm;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{self, Attribute, Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::{self, size, SetSize},
    ExecutableCommand, QueueableCommand,
};
use std::io::{self, Error, Result, Write};
use std::time;

fn redraw() -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout.execute(cursor::MoveTo(5, 5))?;
    // save and later restore the curser pos, as it could break the render
    stdout
        .execute(cursor::SavePosition)?
        .execute(cursor::DisableBlinking)?
        .execute(cursor::Hide)?
        .execute(terminal::Clear(terminal::ClearType::FromCursorUp))?;

    stdout.write(b"Test text lorem ipsum color amet")?;
    stdout.flush()?;

    stdout
        .execute(cursor::RestorePosition)?
        .execute(cursor::EnableBlinking)?
        .execute(cursor::Show)?;
    Ok(())
}

fn print_char(c: char) {
    let mut color = Color::Red;
    if c == 'a' {
        color = Color::Green;
    }
    execute!(
        io::stdout(),
        SetForegroundColor(color),
        Print(format!("{}", c).to_string()),
        ResetColor
    );
}

fn handle_key(key_event: KeyEvent) {
    // TODO: add error handling, somehow to be ok with return types in the match in main
    let mut stdout = io::stdout();
    let code = key_event.code;

    terminal::disable_raw_mode();
    stdout.execute(cursor::Show);

    match code {
        KeyCode::Char(c) => print_char(c),
        _ => (),
    };
    terminal::enable_raw_mode();
}

fn execute_command() {
    ()
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let (cols, rows) = size()?;
    let exit = false;
    let one_sec = time::Duration::from_secs(1);

    let position = 0;

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    // .execute(cursor::DisableBlinking)?
    // .execute(cursor::Hide)?;

    // TODO: is raw mode needed? It would be nicer with it, but it causes the one key behing
    // priting
    terminal::enable_raw_mode()?;

    loop {
        // redraw()?;
        let ev = event::read().unwrap();
        match ev {
            // overloading ^C to break out of the loop and exit
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => break,
            // overloading ^D to break out of the loop and exit
            Event::Key(KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => break,
            // overloading `:` for vim-like commands
            Event::Key(KeyEvent {
                code: KeyCode::Char(':'),
                modifiers: KeyModifiers::NONE,
                ..
            }) => execute_command(),
            // other keys with no modifiers handling
            Event::Key(k) => handle_key(k),
            _ => (),
        }
    }
    // terminal::disable_raw_mode()?;
    Ok(())
}
