use crate::text::wordlist_types::*;

// TODO: Add generation based on word repetition
pub fn run(chars: Vec<char>) -> String {
    let mut wordlist = WordList::new("/home/martinw/Documents/typer/wordlist.txt".to_string());
    wordlist.get_frase(chars)
}