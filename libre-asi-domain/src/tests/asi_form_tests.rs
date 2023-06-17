use crate::models::asi_form::{AsiForm, AsiVersion, FormSection};
use crate::models::question::{Question, QuestionType};

#[test]
fn section_limits() {
    let form = AsiForm::new(AsiVersion::ADMISSION, 6, vec![]);

    assert_eq!(form.check_limits("I14"), "H1");
    assert_eq!(form.check_limits("H12"), "M1");
    assert_eq!(form.check_limits("M28B"), "E1");
    assert_eq!(form.check_limits("E36"), "D1");
    assert_eq!(form.check_limits("D60"), "L1");
    assert_eq!(form.check_limits("L32B"), "F1");
    assert_eq!(form.check_limits("F54"), "P1");
    assert_eq!(form.check_limits("P21"), "V1");
}

#[test]
fn autofill() {
    let question = Question::new(
        "A11",
        "NONE",
        "A14",
        1,
        FormSection::INFO,
        QuestionType::MUL,
        vec![],
    );

    let form = AsiForm::new(AsiVersion::ADMISSION, 6, vec![question]);

    let question = form.question_at(0);

    match question {
        Some(q) =>{
            let result = form.check_autofill(&q, 180);

            assert_eq!(result, "A14");
        },
        None => {
            panic!("form doesn't contain any question");
        }
    }


}
