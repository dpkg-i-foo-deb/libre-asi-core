use super::answer::Answer;

pub struct Interview {
    answers: Vec<Answer>,
}

impl Interview {
    pub fn new(answers: Vec<Answer>) -> Interview {
        Interview { answers }
    }
}

pub struct InterviewResult {
    drug: f32,
    family_child: f32,
    alcohol: f32,
    psych: f32,
    medical: f32,
    legal: f32,
    employment: f32,
    family_social_support: f32,
    family_social_problem: f32,
}
