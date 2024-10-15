// https://leetcode.com/problems/separate-black-and-white-balls/description/?envType=daily-question&envId=2024-10-15
pub fn minimum_steps_i(s: String) -> i64 {
    let s = s.as_bytes();
    let mut white_position = 0;
    let mut total_swaps = 0;

    for current_pos in 0..s.len() {
        if s[current_pos] == '0' as u8 {
            total_swaps += current_pos as i64 - white_position as i64;
            white_position += 1;
        }
    }

    total_swaps
}

pub fn minimum_steps_ii(s: String) -> i64 {
    let s = s.chars().collect::<Vec<_>>();
    let mut total_swaps = 0;
    let mut black_ball_count = 0;

    for i in 0..s.len() {
        if s[i] == '0' {
            total_swaps += black_ball_count;
        } else {
            black_ball_count += 1;
        }
    }
    total_swaps
}
