# Typer

Typer is a small command line utility to help you get better at typing! 

It's written in rust (rust btw), and supports multiple modes of use. Based on the [crossterm](https://github.com/crossterm-rs/crossterm) crate, it's fast and multiplatform.

## Features

  Typer has currently three modes of use: 
  - **frase**, where you can supply your own frase to type, can also be read from `stdin`.
  - **letter**, where random (nonsense) letter words are generated from the supplied chars.
  - **wordlist**, where you can supply letters to focus on, or just practice all with frases containing words from a wordlist.

## Config
  
  While the config is pretty small, it's where you can **set your own wordlist**. It looks for the config file at **`$XDG_CONFIG_HOME`**, which is by default `~/.config/typer/conf.toml`

## Building 

  To build typer, you need to have the [rust toolchain](https://rustup.rs/) installed.
```sh 
  git clone https://github.com/MartinWencl/typer.git
  cd typer
  cargo build --release
```
