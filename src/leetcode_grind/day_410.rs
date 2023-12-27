// https://leetcode.com/problems/minimum-time-to-make-rope-colorful/description
pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
    let mut colors = colors.chars().collect::<Vec<_>>();
    let mut total_time = 0;
    let mut curr_max_time = 0;

    for i in 0..colors.len() {
        if i > 0 && colors[i] != colors[i - 1] {
            curr_max_time = 0;
        }
        total_time += curr_max_time.min(needed_time[i]);
        curr_max_time = curr_max_time.max(needed_time[i]);
    }

    total_time
}
