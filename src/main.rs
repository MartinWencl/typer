#[macro_use]
extern crate crossterm;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{self, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, size},
    ExecutableCommand, QueueableCommand,
};
use std::io::{self, Error, Result, Write};

#[macro_use]
extern crate clap;
use clap::{command, Args, Parser};

#[derive(Parser)]
struct MyArgs {
    frase: String,
}

#[derive(Clone, Debug)]
struct Frase {
    chars: String,
    current: usize,
}

impl Frase {
    fn new(s: &str) -> Self {
        Self {
            chars: String::from(s),
            current: 0,
        }
    }

    fn current_char(&self) -> char {
        let mut char = self.chars.chars().nth(self.current).unwrap();
        if char == ' ' {
            char = '·';
        }
        char
    }

    fn check_char(&mut self, c: char) -> bool {
        if c == self.chars.chars().nth(self.current).unwrap() {
            return true;
        }
        return false;
    }

    fn increment(&mut self) {
        self.current += 1;
    }
}

fn print_char(c: char, frase: &mut Frase) -> Result<()> {
    let mut stdout = io::stdout();
    let mut c = c;

    if frase.check_char(c) {
        stdout.execute(cursor::MoveTo(
            1 + <usize as TryInto<u16>>::try_into(frase.current).unwrap(),
            0,
        ))?;
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Green),
            Print(format!("{}", c).to_string()),
            ResetColor
        )?;
        frase.increment();
    } else {
        c = frase.current_char();

        stdout.execute(cursor::SavePosition)?;
        stdout.execute(cursor::MoveTo(
            1 + <usize as TryInto<u16>>::try_into(frase.current).unwrap(),
            0,
        ))?;
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Red),
            SetBackgroundColor(Color::Red),
            Print(format!("{}", c).to_string()),
            ResetColor
        )?;
        stdout.execute(cursor::RestorePosition)?;
    }
    Ok(())
}

fn handle_key(key_event: KeyEvent, frase: &mut Frase) -> Result<()>{
    // TODO: add error handling, somehow to be ok with return types in the match in main
    let mut stdout = io::stdout();
    let code = key_event.code;

    terminal::disable_raw_mode()?;
    stdout.execute(cursor::Show)?;

    match code {
        KeyCode::Char(c) => print_char(c, frase)?,
        _ => (),
    };
    terminal::enable_raw_mode()?;
    Ok(())
}

fn execute_command() {
    ()
}

fn main() -> io::Result<()> {
    let args = MyArgs::parse();

    let mut stdout = io::stdout();
    // let (cols, rows) = size()?;

    let mut frase = Frase::new(&args.frase);

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::SavePosition)?;
    stdout.execute(cursor::MoveTo(1, 0))?;
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Magenta),
        Print(format!("{}", frase.chars.replace(" ", "·")).to_string()),
        ResetColor
    )?;
    stdout.execute(cursor::RestorePosition)?;

    terminal::enable_raw_mode()?;

    loop {
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
            Event::Key(k) => handle_key(k, &mut frase).unwrap(),
            _ => (),
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}
