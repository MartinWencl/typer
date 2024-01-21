use std::collections::HashMap;

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
}

impl Word {
    /// Returns a map of given letters and a number of their occurences
    pub fn get_occorunces(&self, chars: Vec<char>) -> HashMap<char, u32> {
        let mut result = HashMap::new();
        for char in self.text.chars()   {
            if chars.contains(&char) {
                *result.entry(char).or_insert(0) += 1;
            }
        }
        return result;
    }
}

pub struct WordList {
    words: Vec<Word>,

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
