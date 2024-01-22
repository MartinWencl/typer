use std::{collections::{HashMap, BinaryHeap}, ops::Add, fs::File, io::Read, path::Path};

/// Carries the stats of each letter and acompanied support functionality
pub struct LetterStats {
    pub letter: char,
    pub mistakes: u32,
    pub successes: u32,
    last_succes_precentage: f32,
    last_mistakes: u32,
    last_successes: u32,
}

impl LetterStats {
    /// Retuns the success rete as a precentage
    ///
    /// We check if the number of last mistakes or last successes has changed.
    /// Otherwise no need to recalcuate for this letter
    pub fn get_succes_precentage(&self) -> f32 {
        // This is more for fun than anything else, in reality it probably doesnt save any time
        if (self.mistakes == self.last_mistakes) & (self.successes == self.last_successes) {
            return self.last_succes_precentage;
        }

        return self.successes as f32 / (self.mistakes + self.successes) as f32;
    }
}

/// The "alphabet" here is a set of chars which are to be tracked and prioritized accordingly.
pub struct AlphabetStats {
    pub alphabet: Vec<LetterStats>,
}

impl AlphabetStats {
    /// Returns the success rate as a precentage over the whole alphabet
    ///
    /// For the time being it is implied that atleast one character has changed
    /// and we thus recalculate each call.
    pub fn get_succes_precentage(&self) -> f32 {
        let mut total_perc = 0.0;
        let mut numberof = 0;

        self.alphabet.iter().map(|letter| {
            total_perc += letter.get_succes_precentage();
            numberof += 1;
        });
        return total_perc / numberof as f32;
    }

    pub fn get_haracters(&self) -> Vec<char> {
        let mut chars = Vec::new();
        self.alphabet.iter().map(|c| chars.push(c.letter));
        chars
    }
}

/// Carries the string and support functionality for working with letters on that word
pub struct Word {
    pub text: String,
    pub occurences: u32,
}

impl Ord for Word {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.occurences, &self.text).cmp(&(other.occurences, &other.text))
    }
}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        (self.occurences, &self.text) == (other.occurences, &other.text)
    }
}

impl Eq for Word {
    
}

impl Word {
    pub fn build(text: String, letters: &Vec<char>) -> Self {
        Self {
            occurences : self::Word::get_occurences_integer(text.to_owned(), letters.to_vec()),
            text : text,
        }
    }

    /// Returns a map of given letters and a number of their occurences
    pub fn get_occorunces(word: String, chars: Vec<char>) -> HashMap<char, u32> {
        let mut result = HashMap::new();
        for char in word.chars() {
            if chars.contains(&char) {
                *result.entry(char).or_insert(0) += 1;
            }
        }
        return result;
    }
    
    /// Returns the total number of all occurences of all the given characters
    /// NOTE: Uses get_occurences()
    /// this might change as the total occurences seem to be more important
    /// than per letter occurences
    pub fn get_occurences_integer(word: String, chars: Vec<char>) -> u32 {
        let map = self::Word::get_occorunces(word, chars);
        let mut result = 0;

        map.iter().map(|(_, number_of_occurences)| {
            result += number_of_occurences;
        });
        return result;
    }
}

pub struct WordList {
    words: Vec<Word>,
    path: String,
    // Here build a data structure for returning the words
    // look at trees, where we can get index by number of right letters and total occurences
    // (repetition) in the current frase
    //
    // so that we can just call pop() to get next word
    //
    //  OR
    //
    // the easier version if to generate frases as one whole, not word by word. Then just generate
    // the frase here.
}

impl WordList {
    pub fn new(path: String) -> Self {
        Self {
            path : path,
            words : Vec::new(),
        }
    }

    pub fn get_frase(&mut self, letters: Vec<char>) -> String {
        self.load(letters);
        self.words.sort();
        
        let mut lenght = 0;
        let mut result = String::new();

        for word in &self.words {
            lenght += word.text.len();
            // TODO: Remove magic number and think about how to handle frase len
            if lenght > 300 {
                return result;
            }

            result = result + "" + &word.text;
        };
        return result;
    }

    fn load(&mut self, letters: Vec<char>) {
        let mut wordlist_file = File::open(&self.path).unwrap();
        let mut s = String::new();
        wordlist_file.read_to_string(&mut s);
        let wordlist : Vec<&str> = s.split('\n').collect();

        for word in wordlist {
            self.words.push(Word::build(word.to_owned(), &letters));
        }

    }
}
