use super::asi_form::FormSection;
use super::option::Option;

pub enum QuestionType {
    //Multiple choice
    MUL,
    //Single choice
    SIN,
    //Number input (Integer)
    NUM,
}
//A question from the ASI is represented here
pub struct Question {
    //Here the ID is a string, because every question has a simple
    //identifier in the form, for example... M20,D11,L23...
    id: String,
    //This refers to the question you should skip to in case the
    //conditions are met
    skip_to: String,
    //Refers to which question should be autofilled in case the
    //conditions are met
    autofill_to: String,
    //Order refers to the order the question should appear in A SECTION
    order: i16,
    //The section = Information, medical, employment, etc
    section: FormSection,
    question_type: QuestionType,
    //A question contains multiple options, on the ASI form, they are
    //fixed
    options: Vec<Option>,
}

impl Question {
    pub fn new(
        id: &str,
        skip_to: &str,
        autofill_to: &str,
        order: i16,
        section: FormSection,
        question_type: QuestionType,
        options: Vec<Option>,
    ) -> Question {
        Question {
            id: id.to_string(),
            skip_to: skip_to.to_string(),
            autofill_to: autofill_to.to_string(),
            order,
            section,
            question_type,
            options,
        }
    }

    pub fn section(&self) -> &FormSection {
        &self.section
    }

    pub fn autofill_to(&self) -> &str {
        self.autofill_to.as_str()
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }
}
