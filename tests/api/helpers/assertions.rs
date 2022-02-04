use api_test_client::{LetterValidation, Validity};
use speculoos::{AssertionFailure, Spec};

pub(crate) trait HasValidationResults {
    fn has_validation_results(&mut self, expected_validation_results: Vec<Validity>);
}

impl<'s> HasValidationResults for Spec<'s, Vec<LetterValidation>> {
    fn has_validation_results(&mut self, expected_validation_results: Vec<Validity>) {
        let subject = self.subject;
        let actual: Vec<Validity> = subject
            .iter()
            .map(LetterValidation::validation)
            .copied()
            .collect();

        if expected_validation_results != actual {
            AssertionFailure::from_spec(self)
                .with_expected(format!("{:?}", expected_validation_results))
                .with_actual(format!("{:?}", actual))
                .fail()
        }
    }
}
