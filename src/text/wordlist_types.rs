use std::{fs::File, io::Read};

pub struct Word {
    pub text: String,
    pub occurences: u32,
}

impl Word {
    pub fn build(text: String, letters: &Vec<char>) -> Self {
        Self {
            occurences : Word::get_occurences(text.to_owned(), letters.to_vec()),
            text
        }
    }

    /// Returns the total number of all occurences of all the given characters
    pub fn get_occurences(word: String, chars: Vec<char>) -> u32 {
        let mut result = 0;
        for char in word.chars() {
            if chars.contains(&char) {
                result += 1;
            }
        }
        result
    }
}

pub struct WordList {
    words: Vec<Word>,
    path: String,
}

impl WordList {
    pub fn new(path: String) -> Self {
        Self {
            words : Vec::new(),
            path,
        }
    }

    pub fn get_frase(&mut self, letters: Vec<char>) -> String {
        self.load(letters);
        self.words.sort_unstable_by_key(|a| a.occurences);
        self.words.reverse();

        let mut lenght = 0;
        let mut result = String::new();

        log::info!("Generating frase!");

        for word in &self.words {
            lenght += word.text.len();
            // TODO: Remove magic number and think about how to handle frase len
            if lenght > 300 {
                return result;
            }

            result = result + " " + &word.text;
            log::info!("Adding word {}, with {} occurence/s", word.text, word.occurences);
        };
        log::info!("Frase complete: {}", result.trim().to_string());
        return result.trim().to_string();
    }

    fn load(&mut self, letters: Vec<char>) {
        let mut wordlist_file = File::open(&self.path).unwrap();
        log::info!("Opening file at {}", self.path);

        let mut s = String::new();
        wordlist_file.read_to_string(&mut s).unwrap();
        let wordlist : Vec<&str> = s.split('\n').collect();

        for word in wordlist {
            self.words.push(Word::build(word.to_owned(), &letters));
        }
    }
}
