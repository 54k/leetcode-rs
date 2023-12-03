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

// https://leetcode.com/problems/check-if-it-is-a-straight-line/
pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    fn get_diff_x(a: &Vec<i32>, b: &Vec<i32>) -> i64 {
        a[0] as i64 - b[0] as i64
    }

    fn get_diff_y(a: &Vec<i32>, b: &Vec<i32>) -> i64 {
        a[1] as i64 - b[1] as i64
    }

    let dx = get_diff_x(&coordinates[1], &coordinates[0]);
    let dy = get_diff_y(&coordinates[1], &coordinates[0]);

    for i in 2..coordinates.len() {
        if dy * get_diff_x(&coordinates[i], &coordinates[0])
            != dx * get_diff_y(&coordinates[i], &coordinates[0])
        {
            return false;
        }
    }

    true
}
