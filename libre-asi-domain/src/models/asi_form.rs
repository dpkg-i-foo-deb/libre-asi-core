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

    pub fn new(version:AsiVersion, edition:u8, questions:Vec<Question>) -> AsiForm {
        AsiForm { version, edition, questions }
    }

    pub fn question_at(&self, index:usize) -> Option<&Question> {
        self.questions.get(index)
    }

    pub fn check_limits(&self, question_id: &str) -> &str {
        match question_id {
            "I14" => "H1",
            "H12" => "M1",
            "M28B" => "E1",
            "E36" => "D1",
            "D60" => "L1",
            "L32B" => "F1",
            "F54" => "P1",
            "P21" => "V1",
            "V1" => "#",
            _ => "NONE",
        }
    }


    pub fn check_autofill(&self, question: &Question, answer_value:u8 ) -> String{
        
        match answer_value == 180 {
            true => question.autofill_to().to_owned(),
            false => String::from("#"),
        } 
    }

    pub fn check_dependencies(&self){}
    
}
