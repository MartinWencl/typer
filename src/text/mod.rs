use std::io;
use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};

use crate::types::TextVariants;

mod wordlist_types;

mod wordlist;

const FOX_TEXT : &str = "The quick brown fox jumps over the lazy dog";
const MIN_GENERATED_FRASE_LEN : u8 = 100;
const MAX_GENERATED_FRASE_LEN : u8 = u8::MAX;
const MAX_GENERATED_WORD_LEN : u8 = 8;

pub fn get_text(variant: TextVariants) -> Result<String, io::Error> {
    match variant {
        TextVariants::Letters(vec) => Ok(generate_letter_word(vec)),
        TextVariants::WordList(vec) => Ok(wordlist::run(vec)),
        TextVariants::Test => Ok(FOX_TEXT.to_string()),
    }
}

fn generate_letter_word(letters: Vec<char>) -> String {
    let between = Uniform::from(0..letters.len());
    let mut rng = rand::thread_rng();

    let frase_len = rng.gen_range(MIN_GENERATED_FRASE_LEN..MAX_GENERATED_FRASE_LEN);
    let mut word_len = rng.gen_range(1..MAX_GENERATED_WORD_LEN);
    let mut frase = String::new();

    let mut curr_word_len = 0;
    for _ in 0..frase_len {
        let index = between.sample(&mut rng) % letters.len();
        frase = frase + letters[index].to_string().as_str();

        curr_word_len += 1;
        if curr_word_len >= word_len {
            frase = frase + " ";
            word_len = rng.gen_range(1..MAX_GENERATED_WORD_LEN);
            curr_word_len = 0;
        }
    }
    frase.trim().to_string()
}
