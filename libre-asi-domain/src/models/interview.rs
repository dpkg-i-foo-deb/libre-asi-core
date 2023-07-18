use super::answer::Answer;

pub struct Interview{
    answers: Vec<Answer>
}

impl Interview{
    pub fn new(answers:Vec<Answer>)->Interview{
        Interview { answers }
    }

    pub fn compute_results(){}
}
