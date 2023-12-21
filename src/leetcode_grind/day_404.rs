// https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/description
pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
    let mut points = points;
    points.sort_by_key(|x| x[0]);

    let mut ans = 0;
    for i in 1..points.len() {
        ans = ans.max(points[i][0] - points[i - 1][0]);
    }
    ans
}
