#[derive(derive_new::new, derive_getters::Getters)]
pub struct GuessValidation {
    letters: Vec<LetterValidation>,
}

impl GuessValidation {
    pub fn is_correct(&self) -> bool {
        self.letters.iter().all(LetterValidation::is_correct)
    }

    pub fn guess_string(&self) -> String {
        self.letters.iter().map(|letter| letter.letter()).collect()
    }
}

#[derive(derive_new::new, derive_getters::Getters)]
pub struct LetterValidation {
    letter: char,
    validation: Validity,
}

impl LetterValidation {
    pub fn is_correct(&self) -> bool {
        matches!(self.validation, Validity::Correct)
    }
}

pub enum Validity {
    Correct,
    Incorrect,
    IncorrectPosition,
}
