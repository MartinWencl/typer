#[derive(Debug)]
pub enum InputTypes {
    Unknown,
    Frase(String),
    Letters(String),
    WordList(String),
    Test,
}

#[derive(Debug)]
pub enum TextVariants {
    Letters(Vec<char>),
    WordList(Vec<char>),
    Test,
}
