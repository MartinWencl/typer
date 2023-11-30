#[macro_use]
extern crate crossterm;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{self, Print, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::{
    io::{self, Write},
};
use std::{time};

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

fn handle_key(key_event: KeyEvent) {
    // TODO: add error handling, somehow to be ok with return types in the match in main
    let mut stdout = io::stdout();
    let code = key_event.code;

    terminal::disable_raw_mode();
    stdout.execute(cursor::Show);

    match code {
        KeyCode::Char(c) => stdout.write(&[c as u8]),
        _ => Ok(0),
    };
    terminal::enable_raw_mode();
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let _exit = false;
    let _one_sec = time::Duration::from_secs(1);

    let _position = 0;

    stdout
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(cursor::DisableBlinking)?
        .execute(cursor::Hide)?;

    loop {
        terminal::enable_raw_mode()?;
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
            // TODO: add more control calls
            // i.e reset current line, etc
            Event::Key(KeyEvent {
                code: KeyCode::Char('t'),
                modifiers: KeyModifiers::ALT,
                ..
            }) => execute!(
                stdout,
                terminal::Clear(terminal::ClearType::All),
                Print("crossterm is cool")
            )
            .unwrap(),
            Event::Key(k) => handle_key(k),
            _ => (),
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}
