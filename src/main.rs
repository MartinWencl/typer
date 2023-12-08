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

use clap::Parser;

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

#[derive(Debug)]
struct Stats {
    mistakes: u32,
}


fn print_char(c: char, frase: &mut Frase, stats: &mut Stats) -> Result<()> {
    let mut stdout = io::stdout();
    let mut c = c;
    let (col, row) = cursor::position()?;

    if frase.check_char(c) {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Green),
            Print(format!("{}", c).to_string()),
            ResetColor
        )?;
        stdout.execute(cursor::MoveTo(col + 1, row))?;
        frase.increment();
    } else {
        c = frase.current_char();
        stats.mistakes += 1;

        execute!(
            io::stdout(),
            SetBackgroundColor(Color::DarkRed),
            SetForegroundColor(Color::DarkRed),
            Print(format!("{}", c).to_string()),
            ResetColor
        )?;
        stdout.execute(cursor::MoveTo(col, row))?;
    }
    Ok(())
}

fn handle_key(key_event: KeyEvent, frase: &mut Frase, stats: &mut Stats) -> Result<()>{
    let mut stdout = io::stdout();
    let code = key_event.code;

    terminal::disable_raw_mode()?;
    stdout.execute(cursor::Show)?;

    match code {
        KeyCode::Char(c) => print_char(c, frase, stats)?,
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
    let mut stats = Stats { mistakes : 0 };

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::MoveTo(1, 1))?;
    execute!(
        io::stdout(),
        SetForegroundColor(Color::White),
        Print(format!("{}", frase.chars.replace(" ", "·")).to_string()),
        ResetColor
    )?;
    stdout.execute(cursor::MoveTo(1, 1))?;

    terminal::enable_raw_mode()?;

    loop {
        stdout.execute(cursor::SavePosition)?;
        stdout.execute(cursor::MoveTo(1, 0))?;
        execute!(
            io::stdout(),
            SetForegroundColor(Color::White),
            Print(format!("Mistakes: {}", &stats.mistakes.to_string())),
            ResetColor
        )?;
        stdout.execute(cursor::RestorePosition)?;
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
            Event::Key(k) => handle_key(k, &mut frase, &mut stats).unwrap(),
            _ => (),
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}
