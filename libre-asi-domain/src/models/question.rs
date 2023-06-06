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
pub struct Question<'a> {
    //Here the ID is a string, because every question has a simple
    //identifier in the form, for example... M20,D11,L23...
    id: &'a str,
    //This refers to the question you should skip to in case the
    //conditions are met
    skip_to:&'a str,
    //Refers to which question should be autofilled in case the
    //conditions are met
    autofill_to:&'a str,
    //Order refers to the order the question should appear in A SECTION
    order: i16,
    //The section = Information, medical, employment, etc
    section: FormSection,
    question_type: QuestionType,
    //A question contains multiple options, on the ASI form, they are
    //fixed
    options:Vec<Option>
}

impl Question<'_> {
    pub fn new<'a>(
        id: &'a str,
        skip_to:&'a str,
        autofill_to:&'a str,
        order: i16,
        section: FormSection,
        question_type: QuestionType,
        options:Vec<Option>
    ) -> Question<'a> {
        Question {
            id,
            skip_to,
            autofill_to,
            order,
            section,
            question_type,
            options
        }
    }

    pub fn section(&self) -> &FormSection {
        &self.section
    }

    pub fn autofill_to(&self) -> &str{
        &self.autofill_to
    }
}

