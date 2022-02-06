#[derive(derive_new::new, derive_getters::Getters)]
pub struct GuessValidation {
    letters: Vec<LetterValidation>,
}

impl GuessValidation {
    pub fn guess_string(&self) -> String {
        self.letters.iter().map(|letter| letter.letter()).collect()
    }
}

#[derive(derive_new::new, derive_getters::Getters)]
pub struct LetterValidation {
    letter: char,
    validation: Validity,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Validity {
    Correct,
    Incorrect,
    IncorrectPosition,
}
