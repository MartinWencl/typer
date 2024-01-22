use std::io::{self, Write};

use crossterm::{cursor, style, QueueableCommand};

use crate::text::wordlist_types::*;

pub fn run() -> String {
    // input must be able tu take stats - empty stats mean new start
    // input must be able tu take letters to prioritize

    // TODO: Fix
    let mut wordlist = WordList::new("/home/martinw/Documents/typer/wordlist.txt".to_string());
    let mut stdout = io::stdout();
    let s = wordlist.get_frase(Vec::from(['a']));


    stdout
        .queue(cursor::SavePosition)
        .unwrap()
        .queue(cursor::MoveTo(0, 4))
        .unwrap()
        .queue(style::Print(format!("Frase: {}", s)))
        .unwrap()
        .queue(cursor::RestorePosition)
        .unwrap();
    stdout.flush().unwrap();
    s
    }
