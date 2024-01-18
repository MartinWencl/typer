use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{self, Color, Print},
    terminal::{self, size},
    QueueableCommand,
};
use std::io::{self, Result, Write};

#[derive(Debug)]
pub struct Frase {
    chars: String,
    /// The current character pointer
    current: usize,
}

#[derive(Debug)]
pub struct Stats {
    pub mistakes: u32,
}

impl Frase {
    /// Construct's Frase from the given string slice
    pub fn new(s: &str) -> Self {
        Self {
            chars: String::from(s),
            current: 0,
        }
    }

    /// Return's character at the position of the current character pointer
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

    /// Check's for the correctness of the supplied character by comparing with the character at the
    /// position of the current character pointer
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

    /// Check's if the frase is over by comparing with the current character pointer
    fn is_over(&self) -> bool {
        if self.current >= self.chars.chars().count() {
            return true;
        }
        return false;
    }

    /// Incremet's the current character pointer
    fn increment(&mut self) {
        self.current += 1;
    }
}

/// Print's the correct character in the frase - colored green or red based on comparision with the
/// supplied char `c`.
/// updates frase (current character pointer) and stats accordingly
fn queue_char_printing(key_event: KeyEvent, frase: &mut Frase, stats: &mut Stats) -> Result<()> {
    // Get the character itself
    let mut c;
    match key_event.code {
        KeyCode::Char(char) => c = char,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Unexpected KeyCode! Not a Char.",
            ))
        }
    };

    let mut stdout = io::stdout();
    let (cols, _) = size()?;
    let (curr_col, curr_row) = cursor::position()?;
    let mut move_cursor = 0;

    let is_correct;
    match frase.check_char(c) {
        Some(value) => is_correct = value,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "The current char in the frase was not found!",
            ))
        }
    }

    // Correct: move cursor and increment frase pointer, set color
    // Wrong: get correct char, increment mistakes, set color
    if is_correct {
        move_cursor = 1;
        frase.increment();
        stdout.queue(style::SetForegroundColor(Color::Green))?;
    } else {
        match frase.current_char() {
            Some(value) => c = value,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "The current char in the frase was not found!",
                ))
            }
        }
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

fn queue_mistake_counter(stats: &Stats) -> Result<()> {
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

pub fn run(frase: &mut Frase, stats: &mut Stats) -> Result<()> {
    let mut stdout = io::stdout();

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
    queue_mistake_counter(&stats)?;
    stdout.flush()?;

    loop {
        // Check if we are at the end
        if frase.is_over() {
            break;
        }

        queue_mistake_counter(&stats)?;

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
            Event::Key(k) => queue_char_printing(k, frase, stats)?,
            _ => (),
        }
        // flush all the queried commands from this loops iteration
        stdout.flush()?;
    }
    Ok(())
}
