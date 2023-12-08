#[macro_use]
extern crate crossterm;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{self, Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor},
    terminal::{self, size},
    ExecutableCommand, QueueableCommand,
};
use std::io::{self, Error, Result, Write};

#[macro_use]
extern crate clap;
use clap::{Parser, command, Args};

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
        self.chars.chars().nth(self.current).unwrap()
    }

    fn check_char(&mut self, c :char) -> bool {
       if c == self.current_char() {
            self.current += 1;
            return true;
        } 
        return false;
    }
}

fn print_char(c: char, frase: &mut Frase) {
    let mut stdout = io::stdout();
    let mut color = Color::Red;
    let mut c = c;
    let mut offset = 0;
    let mut background_color = Color::Black;
    if frase.check_char(c) {
        color = Color::Green;
    }
    else {
        c = frase.current_char();
        offset = 1;
        background_color = Color::Red;
        color = Color::Black
    }
    
    stdout.execute(cursor::MoveTo(offset + <usize as TryInto<u16>>::try_into(frase.current).unwrap(), 0));
    execute!(
        io::stdout(),
        SetForegroundColor(color),
        SetBackgroundColor(background_color),
        Print(format!("{}", c).to_string()),
        ResetColor
    );
}

fn handle_key(key_event: KeyEvent, frase: &mut Frase) {
    // TODO: add error handling, somehow to be ok with return types in the match in main
    let mut stdout = io::stdout();
    let code = key_event.code;

    terminal::disable_raw_mode();
    stdout.execute(cursor::Show);

    match code {
        KeyCode::Char(c) => print_char(c, frase),
        _ => (),
    };
    terminal::enable_raw_mode();
}

fn execute_command() {
    ()
}

fn main() -> io::Result<()> {
    let args = MyArgs::parse();

    let mut stdout = io::stdout();
    let (cols, rows) = size()?;

    let mut frase = Frase::new(&args.frase);


    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::SavePosition);
    stdout.execute(cursor::MoveTo(1, 0));
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Magenta),
        Print(format!("{}", frase.chars).to_string()),
        ResetColor
    );
    stdout.execute(cursor::RestorePosition);
    // .execute(cursor::DisableBlinking)?
    // .execute(cursor::Hide)?;

    // TODO: is raw mode needed? It would be nicer with it, but it causes the one key behing
    // priting
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
            Event::Key(k) => handle_key(k, &mut frase),
            _ => (),
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}
