extern crate crossterm;

use crossterm::terminal;
use std::io;
use clap::Parser;

mod frase;
use crate::frase::{Frase, Stats, run};

#[derive(Parser)]
struct MyArgs {
    frase: String,
}

fn main() -> io::Result<()> {
    let args = MyArgs::parse();
    let mut stdout = io::stdout();
    let mut frase = Frase::new(&args.frase);
    let mut stats = Stats { mistakes: 0 };
  
    terminal::enable_raw_mode()?;

    // Run
    let result = run(&mut frase, &mut stats);

    terminal::disable_raw_mode()?;
    result
}
