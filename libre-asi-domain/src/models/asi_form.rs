use super::question::Question;

//This enum represents the multiple sections the ASI form
//contains
pub enum FormSection {
    //Information
    INFO,
    //Housing
    HOUS,
    //Medical
    MED,
    //Employment
    EMP,
    //Drugs and alcohol
    DRU,
    //Laws
    LAW,
    //Family and support
    FAM,
    //Psychological
    PSY,
}

//Here we represent the editions of the ASI form
pub enum AsiVersion {
    ADMISSION,
    FOLLOWUP,
}

//Here we represent the ASI form on an specific edition and version
pub struct AsiForm {
    //Admission, follow-up, etc
    version: AsiVersion,
    //Currently we have the 6th edition
    edition: u8,
    questions: Vec<Question>,
}

impl AsiForm {
    pub fn new(version: AsiVersion, edition: u8, questions: Vec<Question>) -> AsiForm {
        AsiForm {
            version,
            edition,
            questions,
        }
    }

    pub fn question_at(&self, index: usize) -> Option<&Question> {
        self.questions.get(index)
    }

    pub fn check_limits(&self, question_id: &str) -> Option<&str> {
        match question_id {
            "I14" => Some("H1"),
            "H12" => Some("M1"),
            "M28B" => Some("E1"),
            "E36" => Some("D1"),
            "D60" => Some("L1"),
            "L32B" => Some("F1"),
            "F54" => Some("P1"),
            "P21" => Some("V1"),
            _ => None,
        }
    }

    pub fn check_autofill(&self, question: &Question, answer_value: u8) -> Option<String> {
        match answer_value == 180 {
            true => Some(question.autofill_to().to_owned()),
            false => None,
        }
    }

    pub fn check_dependencies(&self) {}
}
