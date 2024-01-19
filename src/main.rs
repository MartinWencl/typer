extern crate crossterm;

use crossterm::terminal;
use std::io;
use clap::Parser;

mod frase;
use crate::frase::{Frase, Stats, run};

mod text;
use crate::text::{get_text, TextVariants};

#[derive(Debug)]
enum InputTypes {
    Unknown,
    Frase(String),
    Letters(String),
}

#[derive(Parser, Debug)]
#[clap(author="Martin Wencl", version="0.0.1", about, long_about = None)]
/// A simple tool for learning typing
struct MyArgs {
    #[clap(short, long, group="input")]
    frase: Option<String>,

    #[clap(short, long, group="input")]
    letters: Option<String>,
}

fn main() -> io::Result<()> {
    let args = MyArgs::parse();
    let mut input_type = InputTypes::Unknown;

    if let Some(s) = &args.frase {
        input_type = InputTypes::Frase(s.to_string());
    }
    if let Some(s) = &args.letters {
        input_type = InputTypes::Letters(s.to_string());
    }

    let mut frase_str = String::new();

    match input_type {
        InputTypes::Frase(s) => frase_str = s,
        InputTypes::Letters(s) => frase_str = get_text(TextVariants::Letters(s.chars().collect())).unwrap(),
        _ => (),
    }
    
    let mut frase = Frase::new(&frase_str);
    let mut stats = Stats { mistakes: 0 };
  
    terminal::enable_raw_mode()?;

    // Run
    // TODO: Repeat run, util the user quits
    let result = run(&mut frase, &mut stats);

    terminal::disable_raw_mode()?;
    result
}
