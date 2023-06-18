use crate::models::asi_form::{AsiForm, AsiVersion, FormSection};
use crate::models::question::{Question, QuestionType};

#[test]
fn section_limits() {
    let form = AsiForm::new(AsiVersion::ADMISSION, 6, vec![]);

    let result = form.check_limits("I14");

    match result {
        Some(id) => assert_eq!(id, "H1"),
        None => panic!("I14 should be a limit"),
    }

    let result = form.check_limits("H12");

    match result {
        Some(id) => assert_eq!(id, "M1"),
        None => panic!("H12 should be a limit"),
    }

    let result = form.check_limits("M28B");

    match result {
        Some(id) => assert_eq!(id, "E1"),
        None => panic!("M28B should be a limit"),
    }

    let result = form.check_limits("E36");

    match result {
        Some(id) => assert_eq!(id, "D1"),
        None => panic!("E36 should be a limit"),
    }

    let result = form.check_limits("D60");

    match result {
        Some(id) => assert_eq!(id, "L1"),
        None => panic!("D60 should be a limit"),
    }

    let result = form.check_limits("L32B");

    match result {
        Some(id) => assert_eq!(id, "F1"),
        None => panic!("L32B should be a limit"),
    }

    let result = form.check_limits("F54");

    match result {
        Some(id) => assert_eq!(id, "P1"),
        None => panic!("F54 should be a limit"),
    }

    let result = form.check_limits("P21");

    match result {
        Some(id) => assert_eq!(id, "V1"),
        None => panic!("P21 should be a limit"),
    }

    let result = form.check_limits("V1");

    match result {
        Some(_) => panic!("V1 should be the end"),
        None => (),
    }
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
        Some(q) => {
            let result = form.check_autofill(&q, 180);

            match result {
                Some(id) => assert_eq!(id, "A14"),
                None => panic!("A11 should have an autofill question"),
            }
        }
        None => {
            panic!("form doesn't contain any question");
        }
    }
}
