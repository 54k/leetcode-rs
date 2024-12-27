// https://leetcode.com/problems/best-sightseeing-pair/description/?envType=daily-question&envId=2024-12-27
pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
    let n = values.len();
    let mut max_left_score = values[0];
    let mut max_score = 0;
    for i in 1..n {
        let current_right_score = values[i] - i as i32;
        max_score = max_score.max(max_left_score + current_right_score);
        let current_left_scope = values[i] + i as i32;
        max_left_score = max_left_score.max(current_left_scope);
    }
    max_score
}
