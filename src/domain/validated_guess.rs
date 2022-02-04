#[derive(derive_new::new, derive_getters::Getters)]
pub(crate) struct ValidatedGuess {
    letters: Vec<ValidatedLetter>,
}

#[derive(derive_new::new, derive_getters::Getters)]
pub(crate) struct ValidatedLetter {
    letter: char,
    validation: Validity,
}

pub(crate) enum Validity {
    Correct,
    Incorrect,
    IncorrectPosition,
}
