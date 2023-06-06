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
    statement: String,
    simplified_tatement: String,
    //Order refers to the order the question should appear in A SECTION
    order: i16,
    //The section = Information, medical, employment, etc
    section: FormSection,
    question_type: QuestionType,
    //A question contains multiple options, on the ASI form, they are
    //fixed
    options:Vec<Option>
}

impl Question {
    pub fn new(
        id: String,
        statement: String,
        simplified_tatement: String,
        order: i16,
        section: FormSection,
        question_type: QuestionType,
        options:Vec<Option>
    ) -> Question {
        Question {
            id,
            statement,
            simplified_tatement,
            order,
            section,
            question_type,
            options
        }
    }
}

