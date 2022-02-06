use api_test_client::{LetterValidation, Validity};
use speculoos::{AssertionFailure, Spec};

pub(crate) trait HasValidationResults {
    fn has_validation_results(&self, expected_validation_results: Vec<Validity>);

    fn has_validity_at_letter_index(&self, index: usize, expected: Validity);
}

impl<'s> HasValidationResults for Spec<'s, Vec<LetterValidation>> {
    fn has_validation_results(&self, expected_validation_results: Vec<Validity>) {
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

    fn has_validity_at_letter_index(&self, index: usize, expected: Validity) {
        let subject = self.subject;

        let actual: &Validity = subject.get(index).unwrap().validation();

        if &expected != actual {
            AssertionFailure::from_spec(self)
                .with_expected(format!("{:?}", expected))
                .with_actual(format!("{:?}", actual))
                .fail()
        }
    }
}
