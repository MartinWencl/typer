use core::fmt;

#[derive(Debug)]
pub enum InputTypes {
    Unknown,
    Frase(String),
    Letters(String),
    WordList(String),
    Test,
}

impl fmt::Display for InputTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputTypes::Frase(s) => write!(f, "Frase: {}", s),
            InputTypes::Letters(s) => write!(f, "Letters: {}", s),
            InputTypes::WordList(s) => write!(f, "WordList: {}", s),
            InputTypes::Test => write!(f, "Test"),
            InputTypes::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug)]
pub enum TextVariants {
    Letters(Vec<char>),
    WordList(Vec<char>),
    Test,
}

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
    pub fn new(letter: char) -> Self {
        Self {
            letter,
            mistakes: 0,
            successes: 0,
            last_successes: 0,
            last_mistakes: 0,
            last_succes_precentage: 1.0,
        }
    }

    /// Retuns the success rete as a precentage
    ///
    /// We check if the number of last mistakes or last successes has changed.
    /// Otherwise no need to recalcuate for this letter
    /// TODO: Fix
    pub fn get_succes_precentage(&self) -> f32 {
        // This is more for fun than anything else, in reality it probably doesnt save any time
        if (self.mistakes == self.last_mistakes) & (self.successes == self.last_successes) {
            return self.last_succes_precentage;
        }

        return self.successes as f32 / (self.mistakes + self.successes) as f32;
    }
}

// TODO: This is litterally a bool
pub enum Outcomes {
    Correct,
    Wrong,
}

/// The "alphabet" here is a set of chars which are to be tracked and prioritized accordingly.
pub struct AlphabetStats {
    pub alphabet: Vec<LetterStats>,
}

impl AlphabetStats {
    pub fn new(chars: Vec<char>) -> Self {
        let mut stats: Vec<LetterStats> = Vec::new();
        for char in chars {
            stats.push(LetterStats::new(char));
        }
        Self { alphabet: stats }
    }

    pub fn increment(&mut self, what: Outcomes, letter: char) {
        for letterstat in &mut self.alphabet {
            if letterstat.letter == letter {
                match what {
                    Outcomes::Wrong => letterstat.mistakes += 1,
                    Outcomes::Correct => letterstat.successes += 1,
                }
            }
        }
    }

    /// Returns the success rate as a precentage over the whole alphabet
    ///
    /// For the time being it is implied that atleast one character has changed
    /// and we thus recalculate each call.
    pub fn get_succes_precentage(&self) -> f32 {
        let mut total_perc = 0.0;
        let mut numberof = 0;

        for letterstat in &self.alphabet {
            if letterstat.mistakes == 0 {
                continue;
            }
            total_perc += letterstat.get_succes_precentage();
            numberof += 1;
        }
        if total_perc == 0.0 {
            return 1.0;
        }
        return total_perc / numberof as f32;
    }

    pub fn get_mistakes(&self) -> u32 {
        let mut total = 0;

        for letter in &self.alphabet {
            total += letter.mistakes;
        }
        total
    }

    pub fn get_characters(&self) -> Vec<char> {
        let mut chars = Vec::new();
        self.alphabet.iter().for_each(|c| chars.push(c.letter));
        chars
    }
}
