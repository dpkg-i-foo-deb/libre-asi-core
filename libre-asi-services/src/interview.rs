use domain::{
    models::{answer::Answer, question::Question},
    repositories::question::QuestionRepository,
};

pub struct InterviewService<Q: QuestionRepository> {
    question_repository: Q,
}

impl<Q: QuestionRepository> InterviewService<Q> {
    pub fn compute_tmraw_results(&self) {}

    fn compute_dru_tmraw(answers: &mut Vec<Answer>) {
        let mut result: f32 = 0.0;

        let mut d25_33dn: u8 = 0;
        let mut d25_33en: u8 = 0;

        loop {
            match answers.pop() {
                Some(answer) => match answer.question_id() {
                    "D25D" | "D26D" | "D27D" | "D28D" | "D29D" | "D30D" | "D31D" | "D32D"
                    | "D33D" => d25_33dn += compute_score_range_type_1(&answer.int_value()),
                    "D25E" | "D26E" | "D27E" | "D28E" | "D29E" | "D30E" | "D31E" | "D32E"
                    | "D33E" => d25_33en += 1,
                    "D39" => result += f32::from(compute_score_range_type_2(&answer.int_value())),
                    "D40" => result += f32::from(compute_score_range_type_3(&answer.int_value())),
                    _ => (),
                },
                None => break,
            };
        }
    }
}

fn compute_score_range_type_1(value: &u16) -> u8 {
    match value {
        0 => 0,
        1..=15 => 1,
        16..=30 => 2,
        31..=60 => 3,
        _ => 4,
    }
}

fn compute_score_range_type_2(value: &u16) -> u8 {
    match value {
        0 => 0,
        1..=5 => 1,
        6..=15 => 2,
        16..=25 => 3,
        _ => 4,
    }
}

fn compute_score_range_type_3(value: &u16) -> u8 {
    match value {
        0..=3 => 4,
        4..=7 => 3,
        8..=14 => 2,
        15..=30 => 1,
        _ => 0,
    }
}

fn compute_score_range_type_4(value: &f32) -> u8 {
    if value == &0.0 {
        return 0;
    }

    if value >= &1.0 && value <= &299999.0 {
        return 1;
    }

    if value >= &3000000.0 && value <= &6000000.0 {
        return 2;
    }

    if value >= &6000001.0 && value <= &1000000.0 {
        return 3;
    }

    if value >= &1000001.0 {
        return 4;
    }

    0
}
