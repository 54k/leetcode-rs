// https://leetcode.com/problems/minimum-time-visiting-all-points/description
pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for i in 0..points.len() - 1 {
        let (curr_x, curr_y) = (points[i][0], points[i][1]);
        let (target_x, target_y) = (points[i + 1][0], points[i + 1][1]);
        ans += (target_x - curr_x).abs().max((target_y - curr_y).abs());
    }
    ans
}
