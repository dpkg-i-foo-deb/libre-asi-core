#[cfg(test)]
mod interview_tests {
    use domain::{
        models::question::Question,
        repositories::{self, question::QuestionRepository},
    };

    use crate::interview::{
        compute_score_range_type_1, compute_score_range_type_2, compute_score_range_type_3,
        compute_score_range_type_4, InterviewService,
    };

    //TODO FIND A WAY TO TEST THIS
    #[test]
    fn tmraw_scores() {
        todo!()
    }

    #[test]
    fn score_range_type_1() {
        assert_eq!(compute_score_range_type_1(&0), 0);
        assert_eq!(compute_score_range_type_1(&1), 1);
        assert_eq!(compute_score_range_type_1(&2), 1);
        assert_eq!(compute_score_range_type_1(&15), 1);
        assert_eq!(compute_score_range_type_1(&16), 2);
        assert_eq!(compute_score_range_type_1(&17), 2);
        assert_eq!(compute_score_range_type_1(&30), 2);
        assert_eq!(compute_score_range_type_1(&31), 3);
        assert_eq!(compute_score_range_type_1(&32), 3);
        assert_eq!(compute_score_range_type_1(&60), 3);
        assert_eq!(compute_score_range_type_1(&61), 4);
        assert_eq!(compute_score_range_type_1(&70), 4);
    }

    #[test]
    fn score_range_type_2() {
        assert_eq!(compute_score_range_type_2(&0), 0);
        assert_eq!(compute_score_range_type_2(&1), 1);
        assert_eq!(compute_score_range_type_2(&2), 1);
        assert_eq!(compute_score_range_type_2(&5), 1);
        assert_eq!(compute_score_range_type_2(&6), 2);
        assert_eq!(compute_score_range_type_2(&7), 2);
        assert_eq!(compute_score_range_type_2(&15), 2);
        assert_eq!(compute_score_range_type_2(&16), 3);
        assert_eq!(compute_score_range_type_2(&17), 3);
        assert_eq!(compute_score_range_type_2(&25), 3);
        assert_eq!(compute_score_range_type_2(&80), 4);
    }

    #[test]
    fn score_range_type_3() {
        assert_eq!(compute_score_range_type_3(&0), 4);
        assert_eq!(compute_score_range_type_3(&1), 4);
        assert_eq!(compute_score_range_type_3(&3), 4);
        assert_eq!(compute_score_range_type_3(&4), 3);
        assert_eq!(compute_score_range_type_3(&5), 3);
        assert_eq!(compute_score_range_type_3(&7), 3);
        assert_eq!(compute_score_range_type_3(&8), 2);
        assert_eq!(compute_score_range_type_3(&9), 2);
        assert_eq!(compute_score_range_type_3(&14), 2);
        assert_eq!(compute_score_range_type_3(&15), 1);
        assert_eq!(compute_score_range_type_3(&16), 1);
        assert_eq!(compute_score_range_type_3(&30), 1);
        assert_eq!(compute_score_range_type_3(&90), 0);
    }

    #[test]
    fn score_range_type_4() {
        assert_eq!(compute_score_range_type_4(&0.0), 0);
        assert_eq!(compute_score_range_type_4(&1.0), 1);
        assert_eq!(compute_score_range_type_4(&299999.0), 1);
        assert_eq!(compute_score_range_type_4(&300000.), 2);
        assert_eq!(compute_score_range_type_4(&600000.0), 2);
        assert_eq!(compute_score_range_type_4(&600001.0), 3);
        assert_eq!(compute_score_range_type_4(&1000000.0), 3);
        assert_eq!(compute_score_range_type_4(&91939452.0), 4);
    }
}
