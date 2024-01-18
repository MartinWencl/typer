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

#[derive(Debug)]
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

    fn current_char(&self) -> Option<char> {
        if let Some(mut char) = self.chars.chars().nth(self.current) {
            if char == ' ' {
                char = '·';
            }
            return Some(char);
        } else {
            return None;
        }
    }

    fn check_char(&mut self, c: char) -> Option<bool> {
        // TODO: There is a better way for sure - check the question mark operator
        if let Some(curr) = self.chars.chars().nth(self.current) {
            if c == curr {
                return Some(true);
            }
            return Some(false);
        } else {
            return None;
        }
    }

    fn is_over(&self) -> bool {
        if self.current >= self.chars.chars().count() {
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

#[derive(Debug)]
struct Cursor {
    row: u16,
    col: u16,
}

fn print_char(c: char, frase: &mut Frase, stats: &mut Stats) -> Result<()> {
    let mut stdout = io::stdout();
    let mut c = c;
    let (cols, rows) = size()?;
    let (mut curr_col, mut curr_row) = cursor::position()?;
    let mut move_cursor = 0;
    let is_correct = frase.check_char(c);

    if is_correct.is_none() {
        // terminal::enable_raw_mode()?;
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "The current char in the frase was not found!",
        ));
    }

    // TODO: Replace this unwrap, even though it should be safe
    if is_correct.unwrap() {
        move_cursor = 1;
        frase.increment();
        stdout.queue(style::SetForegroundColor(Color::Green))?;
    } else {
        let current = frase.current_char();
        if current.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "The current char in the frase was not found!",
            ));
        }
        // TODO: Replace this unwrap, even though it should be safe
        c = current.unwrap();
        stats.mistakes += 1;
        stdout
            .queue(style::SetForegroundColor(Color::DarkRed))?
            .queue(style::SetBackgroundColor(Color::DarkRed))?;
    }
    stdout
        .queue(style::Print(format!("{}", c).to_string()))?
        .queue(cursor::MoveTo(curr_col + move_cursor, curr_row))?;

    // Check if we are at the end of the line!
    if curr_col + move_cursor >= cols {
        stdout.queue(cursor::MoveTo(0, curr_row + 1))?;
    }

    stdout.queue(style::ResetColor)?;
    Ok(())
}

// TODO: Merge into print_char
fn handle_key(key_event: KeyEvent, frase: &mut Frase, stats: &mut Stats) -> Result<()> {
    let code = key_event.code;

    match code {
        KeyCode::Char(c) => print_char(c, frase, stats)?,
        _ => (),
    };
    Ok(())
}

fn print_mistake_counter(stats: &Stats) -> Result<()> {
    let mut stdout = io::stdout();

    stdout
        .queue(cursor::SavePosition)?
        .queue(cursor::MoveTo(1, 0))?
        .queue(style::SetForegroundColor(Color::White))?
        .queue(style::Print(format!(
            "Mistakes: {}",
            stats.mistakes.to_string()
        )))?
        .queue(style::ResetColor)?
        .queue(cursor::RestorePosition)?;
    Ok(())
}

fn execute_command() {
    ()
}

fn run(frase: &mut Frase, stats: &mut Stats) -> Result<()> {
    let mut stdout = io::stdout();

    loop {
        // Check if we are at the end
        if frase.is_over() {
            break;
        }

        print_mistake_counter(&stats)?;

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
            Event::Key(k) => handle_key(k, frase, stats)?,
            _ => (),
        }
        // flush all the queried commands from this loops iteration
        stdout.flush()?;

        // DEBUG - prints column info on the screen
        // let (cols, rows) = size()?;
        // let (curr_col, curr_row) = cursor::position()?;
        // // Check if we are at the end of the line
        // stdout
        // .queue(cursor::SavePosition)?
        // .queue(cursor::MoveTo(1, 5))?
        // .queue(style::SetForegroundColor(Color::White))?
        // .queue(style::Print(format!(
        //     "col: {}, max cols: {}",
        //     curr_col.to_string(),
        //     cols.to_string(),
        //     )))?
        // .queue(style::ResetColor)?
        // .queue(cursor::RestorePosition)?;
        // stdout.flush()?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args = MyArgs::parse();

    let mut stdout = io::stdout();
    // let (cols, rows) = size()?;

    let mut frase = Frase::new(&args.frase);
    let mut stats = Stats { mistakes: 0 };

    // Initiliaze the first screen
    stdout
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0, 1))?
        .queue(style::SetForegroundColor(Color::White))?
        .queue(Print(
            // print the dots only at the initialization, the String in frase is still with spaces
            // This means that once you get past the given space, the dot is again replaced
            // with a space
            // TODO: think about if I want to keep it that way or print dots when typed correctly
            format!("{}", frase.chars.replace(" ", "·")).to_string(),
        ))?
        .queue(style::ResetColor)?
        .queue(cursor::MoveTo(0, 1))?;

    // Print the empty counter as part of first screen initialization
    print_mistake_counter(&stats)?;
    stdout.flush()?;

    terminal::enable_raw_mode()?;

    // Run
    let result = run(&mut frase, &mut stats);

    terminal::disable_raw_mode()?;
    result
}
