use super::question::Question;

pub struct Answer {
    question: Question,
    value: f32,
    comment: String,
}

impl Answer {
    pub fn question_id(&self) -> &str {
        &self.question.id()
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn int_value(&self) -> u16 {
        self.value as u16
    }
}
