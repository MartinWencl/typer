use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{self, Color, Print},
    terminal::{self, size},
    QueueableCommand,
};
use std::io::{self, Result, Write};
use crate::types::{AlphabetStats, Outcomes};

#[derive(Debug)]
pub struct Frase {
    chars: String,
    /// The current character pointer
    current: usize,
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
fn queue_char_printing(key_event: KeyEvent, frase: &mut Frase, stats: &mut AlphabetStats) -> Result<()> {
    // Get the character itself from the KeyEvent
    let mut c;
    match key_event.code {
        KeyCode::Char(char) => c = char,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unexpected KeyCode! Not a Char.",
            ))
        }
    };

    log::info!("Recieved the char: \"{}\"", c);

    let mut stdout = io::stdout();
    let (cols, _) = size()?;
    let (curr_col, curr_row) = cursor::position()?;

    // Variable storing the cursor movement, if the given char (pressed key) is correct,
    // then we move, otherwise the cursor stays on the same char.
    let mut move_cursor = 0;

    // Set `is_correct` according to the given char (pressed key)
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
        stats.increment(Outcomes::Correct, c);
        stdout.queue(style::SetForegroundColor(Color::Green))?;
    } else {
        // Get current char from frase - replace given char 
        // we want to print the CORRECT char with red coloring
        match frase.current_char() {
            Some(value) => c = value,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "The current char in the frase was not found!",
                ))
            }
        }
        stats.increment(Outcomes::Wrong, c);
        stdout
            .queue(style::SetForegroundColor(Color::DarkRed))?
            .queue(style::SetBackgroundColor(Color::DarkRed))?;
    }

    // Reprint the char, always should be the correct one, just colored accordingly
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

fn queue_mistake_counter(stats: &AlphabetStats) -> Result<()> {
    let mut stdout = io::stdout();

    stdout
        .queue(cursor::SavePosition)?
        .queue(cursor::MoveTo(1, 0))?
        .queue(style::SetForegroundColor(Color::White))?
        .queue(style::Print(format!(
            "Mistakes: {}  Percentage: {}",
            stats.get_mistakes().to_string(),
            stats.get_succes_precentage().to_string()
        )))?
        .queue(style::ResetColor)?
        .queue(cursor::RestorePosition)?;
    Ok(())
}

pub fn run(frase: &mut Frase, stats: &mut AlphabetStats) -> Result<()> {
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
    log::info!("Printing frase: {}", frase.chars.replace(" ", "·").to_string());

    // Print the empty counter as part of first screen initialization
    queue_mistake_counter(&stats)?;
    stdout.flush()?;

    loop {
        // Check if we are at the end
        if frase.is_over() {
            break;
        }

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
            // TODO: overloading `:` for vim-like commands
            Event::Key(KeyEvent {
                code: KeyCode::Char(':'),
                modifiers: KeyModifiers::NONE,
                ..
            }) => (),
            // other keys with no modifiers handling
            Event::Key(k) => queue_char_printing(k, frase, stats)?,
            _ => (),
        }
        queue_mistake_counter(&stats)?;
        // flush all the queried commands from this loops iteration
        stdout.flush()?;
    }
    Ok(())
}